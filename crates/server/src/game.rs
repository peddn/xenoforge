//! Playing in a team: commands to the rule engine, previews, corrections
//! and undo.
//!
//! For every command the server puts the engine state together from the
//! team document and the team's characters, lets the engine work, and
//! writes everything back in one transaction. Each value is stored in one
//! place only.

use std::collections::{BTreeMap, HashMap};
use std::sync::PoisonError;

use axum::extract::{Path, State};
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use xenoforge_engine::data::Id;
use xenoforge_engine::data::effects::{Op, Who};
use xenoforge_engine::dice::Die;
use xenoforge_engine::roll::{RollKind, RollRequest, RollState};
use xenoforge_engine::state::{Actor, GameState, Pending, Supply};
use xenoforge_engine::{Choice, Command, Engine, Preview};

use crate::auth::User;
use crate::characters::{self, Character};
use crate::error::{ApiError, ApiResult};
use crate::log::{self, Entry, Stored};
use crate::{AppState, begin_write, now};

/// How many steps the game master can take back.
const UNDO_DEPTH: i64 = 20;

/// A team row; `state` is the engine state without actors.
#[derive(Debug, Clone)]
pub struct Team {
    pub id: i64,
    pub gm_id: i64,
    pub name: String,
    pub invite_token: String,
    pub state: GameState,
    /// Agendas per Act by character id, in Cinematic play (A-13).
    pub agendas: BTreeMap<i64, Vec<String>>,
}

fn team_from_row(row: &sqlx::sqlite::SqliteRow) -> ApiResult<Team> {
    let state: String = row.get("state");
    let agendas: String = row.get("agendas");
    Ok(Team {
        id: row.get("id"),
        gm_id: row.get("gm_id"),
        name: row.get("name"),
        invite_token: row.get("invite_token"),
        state: serde_json::from_str(&state)?,
        agendas: serde_json::from_str(&agendas)?,
    })
}

impl Team {
    /// The Agenda of the current Act, if the game master set one.
    pub fn agenda_now(&self, character: i64) -> Option<String> {
        let act = usize::from(self.state.acts_ended);
        self.agendas
            .get(&character)
            .and_then(|list| list.get(act))
            .filter(|text| !text.is_empty())
            .cloned()
    }
}

pub async fn load_team(pool: &SqlitePool, id: i64) -> ApiResult<Team> {
    let row = sqlx::query("SELECT * FROM teams WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::not_found("team"))?;
    team_from_row(&row)
}

/// Writes the team's Agendas per Act (A-13).
pub async fn save_agendas(tx: &mut Transaction<'_, Sqlite>, team: &Team) -> ApiResult<()> {
    sqlx::query("UPDATE teams SET agendas = ? WHERE id = ?")
        .bind(serde_json::to_string(&team.agendas)?)
        .bind(team.id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

async fn load_team_tx(tx: &mut Transaction<'_, Sqlite>, id: i64) -> ApiResult<Team> {
    let row = sqlx::query("SELECT * FROM teams WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or_else(|| ApiError::not_found("team"))?;
    team_from_row(&row)
}

pub async fn team_characters(
    tx: &mut Transaction<'_, Sqlite>,
    team_id: i64,
) -> ApiResult<Vec<Character>> {
    let rows = sqlx::query("SELECT * FROM characters WHERE team_id = ? ORDER BY id")
        .bind(team_id)
        .fetch_all(&mut **tx)
        .await?;
    rows.iter().map(characters::from_row).collect()
}

async fn save_team_state(
    tx: &mut Transaction<'_, Sqlite>,
    team_id: i64,
    state: &GameState,
) -> ApiResult<()> {
    sqlx::query("UPDATE teams SET state = ? WHERE id = ?")
        .bind(serde_json::to_string(state)?)
        .bind(team_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// Is the user the game master of the team? Members and owners of
/// characters in the team get `false`; everyone else is refused (A-07).
pub async fn access(pool: &SqlitePool, team: &Team, user: &User) -> ApiResult<bool> {
    if team.gm_id == user.id {
        return Ok(true);
    }
    let member = sqlx::query("SELECT 1 FROM members WHERE team_id = ? AND user_id = ?")
        .bind(team.id)
        .bind(user.id)
        .fetch_optional(pool)
        .await?
        .is_some();
    if member {
        Ok(false)
    } else {
        Err(ApiError::not_found("team"))
    }
}

/// The engine state: the team document plus the actors of its characters.
/// Story Points belong to the player, so each actor names its owner. The
/// game master's own characters have no player and so no Story Points
/// (A-13).
pub fn assemble(team: &Team, characters: &[Character]) -> GameState {
    let mut game = team.state.clone();
    game.actors = characters
        .iter()
        .map(|c| {
            let mut actor = c.data.actor.clone();
            actor.player = (c.owner_id != team.gm_id).then(|| c.owner_id.to_string());
            (c.id.to_string(), actor)
        })
        .collect();
    game
}

/// Splits the engine state back into team document and characters.
async fn store(
    tx: &mut Transaction<'_, Sqlite>,
    team_id: i64,
    mut game: GameState,
    characters: &mut [Character],
) -> ApiResult<()> {
    for character in characters.iter_mut() {
        if let Some(actor) = game.actors.remove(&character.id.to_string()) {
            character.data.actor = actor;
            characters::save(tx, character).await?;
        }
    }
    game.actors.clear();
    save_team_state(tx, team_id, &game).await
}

/// A state for undo: the team document and the characters' values. Name,
/// appearance and notes are the player's and stay as they are.
#[derive(Serialize, Deserialize)]
struct Snapshot {
    team: GameState,
    characters: Vec<(i64, Actor)>,
}

async fn save_snapshot(
    tx: &mut Transaction<'_, Sqlite>,
    team: &Team,
    characters: &[Character],
) -> ApiResult<()> {
    let snapshot = Snapshot {
        team: team.state.clone(),
        characters: characters
            .iter()
            .map(|c| (c.id, c.data.actor.clone()))
            .collect(),
    };
    sqlx::query("INSERT INTO snapshots (team_id, state, created_at) VALUES (?, ?, ?)")
        .bind(team.id)
        .bind(serde_json::to_string(&snapshot)?)
        .bind(now())
        .execute(&mut **tx)
        .await?;
    sqlx::query(
        "DELETE FROM snapshots WHERE team_id = ? AND id NOT IN \
         (SELECT id FROM snapshots WHERE team_id = ? ORDER BY id DESC LIMIT ?)",
    )
    .bind(team.id)
    .bind(team.id)
    .bind(UNDO_DEPTH)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// The actor whose owner decides a pending decision; `None` means only
/// the game master decides (choosing targets).
fn decider(pending: &Pending) -> Option<&str> {
    match pending {
        Pending::Roll(roll) => roll.request.actor.as_deref(),
        Pending::StartRoll { request, .. } => request.actor.as_deref(),
        Pending::Overkill { actor, .. } | Pending::Question { actor, .. } => Some(actor),
        Pending::Targets { .. } => None,
    }
}

/// Who may give which command: players act for their own characters and
/// answer their own decisions, and move, drop, hand over and use their own
/// items; everything else is the game master's, new items and refilling
/// too.
fn check_permission(
    team: &Team,
    command: &Command,
    user: &User,
    is_gm: bool,
    owners: &HashMap<String, i64>,
) -> ApiResult<()> {
    if is_gm {
        return Ok(());
    }
    let owns = |actor: &str| owners.get(actor) == Some(&user.id);
    let allowed = match command {
        Command::Roll { request } => request.actor.as_deref().is_some_and(owns),
        Command::Decide { pending, .. } => match team.state.pending.get(pending) {
            Some(p) => decider(p).is_some_and(owns),
            // Unknown decisions are reported by the engine.
            None => true,
        },
        Command::UseSignatureItem { actor }
        | Command::PerformAction { actor, .. }
        | Command::MoveItem { actor, .. }
        | Command::UseItem { actor, .. } => owns(actor),
        _ => false,
    };
    if allowed {
        Ok(())
    } else {
        Err(ApiError::forbidden("only the game master can do that"))
    }
}

/// Free text in a command: an item's name as long as a character's, the
/// reason for a booking as long as a note.
pub fn check_texts(command: &Command) -> ApiResult<()> {
    let (text, max) = match command {
        Command::AddItem { item, .. } | Command::BuyItem { item, .. } => {
            (item.name.as_deref(), characters::NAME_MAX)
        }
        Command::Book { note, .. } => (Some(note.as_str()), log::NOTE_MAX),
        _ => return Ok(()),
    };
    if text.is_some_and(|t| t.trim().chars().count() > max) {
        return Err(ApiError::bad_request(format!(
            "that text has at most {max} characters"
        )));
    }
    Ok(())
}

/// What a roll may bring along (A-08): a difficulty from the table on
/// p. 63 and a free gear bonus up to `gear_bonus_max`, each at most once.
/// Both are the game master's: players choose their items instead (F-59).
/// Players roll skill and death rolls only; a roll of an attribute alone
/// has no Stress Dice (F-27), so only the rules or the game master ask for
/// one.
fn check_roll(engine: &Engine, request: &RollRequest, is_gm: bool) -> ApiResult<()> {
    let player_kind = matches!(request.kind, RollKind::Skill { .. } | RollKind::Death);
    if !is_gm && !player_kind {
        return Err(ApiError::forbidden(
            "players roll skill and death rolls (A-08)",
        ));
    }
    let (mut gear, mut difficulty) = (false, false);
    for modification in &request.modifiers {
        let value = i16::from(modification.value);
        if modification.source == "gear" {
            if !is_gm {
                return Err(ApiError::forbidden(
                    "players choose their items; a free gear bonus is the game master's (F-59)",
                ));
            }
            if gear || !(0..=engine.constant("gear_bonus_max")).contains(&value) {
                return Err(ApiError::bad_request(
                    "one gear bonus from 0 to gear_bonus_max (A-08)",
                ));
            }
            gear = true;
            continue;
        }
        let level = engine
            .rules()
            .constants
            .iter()
            .find(|c| c.id.starts_with("difficulty_") && c.id == modification.source)
            .ok_or_else(|| ApiError::bad_request("unknown modification (A-08)"))?;
        if !is_gm {
            return Err(ApiError::forbidden(
                "the game master sets the difficulty (A-08) [p. 63]",
            ));
        }
        if difficulty || level.value != value {
            return Err(ApiError::bad_request(
                "one difficulty with its value from the table [p. 63]",
            ));
        }
        difficulty = true;
    }
    Ok(())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{id}/commands", post(command))
        .route("/{id}/preview", post(preview))
        .route("/{id}/undo", post(undo))
}

#[derive(Deserialize)]
struct CommandBody {
    command: Command,
    /// Why, in the person's words; shown with the command in the log.
    #[serde(default)]
    note: Option<String>,
}

/// Runs an engine command and returns the log entries the user may see.
async fn command(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(body): Json<CommandBody>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let team = load_team(&state.pool, id).await?;
    let is_gm = access(&state.pool, &team, &user).await?;
    let note = log::note(body.note)?;
    check_texts(&body.command)?;
    if let Command::Roll { request } = &body.command {
        check_roll(&state.engine, request, is_gm)?;
    }
    let mut tx = begin_write(&state.pool).await?;
    let team = load_team_tx(&mut tx, id).await?;
    let mut characters = team_characters(&mut tx, id).await?;
    let owners: HashMap<String, i64> = characters
        .iter()
        .map(|c| (c.id.to_string(), c.owner_id))
        .collect();
    check_permission(&team, &body.command, &user, is_gm, &owners)?;
    // Story Points go to players of the team (F-53).
    if let Command::SetStoryPoints { player, .. } = &body.command {
        let member = sqlx::query("SELECT 1 FROM members WHERE team_id = ? AND user_id = ?")
            .bind(id)
            .bind(player)
            .fetch_optional(&mut *tx)
            .await?
            .is_some();
        if !member {
            return Err(ApiError::bad_request(
                "Story Points go to players of the team",
            ));
        }
    }

    let mut game = assemble(&team, &characters);
    let events = {
        // The dice hold no state worth protecting: after a panic elsewhere
        // they still work.
        let mut dice = state.dice.lock().unwrap_or_else(PoisonError::into_inner);
        state.engine.apply(&mut game, body.command, &mut **dice)?
    };
    save_snapshot(&mut tx, &team, &characters).await?;
    store(&mut tx, id, game, &mut characters).await?;
    let entries = events
        .into_iter()
        .map(|event| Entry::Engine { event })
        .collect();
    let stored = log::append(&mut tx, id, &user, entries, note).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(
        stored
            .into_iter()
            .filter(|s| log::visible(s, is_gm))
            .collect(),
    ))
}

/// The pool a roll would have, computed by the engine (plan 11.1: the
/// interface computes no rules).
async fn preview(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(request): Json<RollRequest>,
) -> ApiResult<Json<Preview>> {
    let team = load_team(&state.pool, id).await?;
    let is_gm = access(&state.pool, &team, &user).await?;
    let mut tx = state.pool.begin().await?;
    let characters = team_characters(&mut tx, id).await?;
    tx.rollback().await?;
    let owns = request.actor.as_deref().is_some_and(|a| {
        characters
            .iter()
            .any(|c| c.id.to_string() == a && c.owner_id == user.id)
    });
    if !is_gm && !owns {
        return Err(ApiError::forbidden("not your character"));
    }
    check_roll(&state.engine, &request, is_gm)?;
    let game = assemble(&team, &characters);
    Ok(Json(state.engine.preview(&game, request)?))
}

/// Takes back the last command or correction.
async fn undo(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let team = load_team(&state.pool, id).await?;
    if !access(&state.pool, &team, &user).await? {
        return Err(ApiError::forbidden("only the game master can undo"));
    }
    let mut tx = begin_write(&state.pool).await?;
    let row =
        sqlx::query("SELECT id, state FROM snapshots WHERE team_id = ? ORDER BY id DESC LIMIT 1")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| ApiError::conflict("nothing to undo"))?;
    let mut snapshot: Snapshot = serde_json::from_str(row.get("state"))?;
    // Numbers of decisions and rolls are never handed out twice: the log
    // already names the newer ones.
    snapshot.team.next_pending = snapshot.team.next_pending.max(team.state.next_pending);
    save_team_state(&mut tx, id, &snapshot.team).await?;
    for mut character in team_characters(&mut tx, id).await? {
        if let Some((_, actor)) = snapshot
            .characters
            .iter()
            .find(|(cid, _)| *cid == character.id)
        {
            character.data.actor = actor.clone();
            characters::save(&mut tx, &character).await?;
        }
    }
    sqlx::query("DELETE FROM snapshots WHERE id = ?")
        .bind(row.get::<i64, _>("id"))
        .execute(&mut *tx)
        .await?;
    let stored = log::append(&mut tx, id, &user, vec![Entry::Undone], None).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(stored))
}

#[derive(Deserialize)]
pub struct Correction {
    actor: Actor,
    /// The values as the form showed them when it was opened; if the
    /// character changed since, the correction is refused.
    #[serde(default)]
    base: Option<Actor>,
    #[serde(default)]
    note: Option<String>,
}

/// The game master overwrites a character's values except the items;
/// logged and undoable (plan section 3). With the values the form started
/// from, it is refused when something happened in between, e.g. a player
/// used an item, so nothing is undone unseen.
pub async fn correct(
    State(state): State<AppState>,
    user: User,
    Path((id, character_id)): Path<(i64, i64)>,
    Json(body): Json<Correction>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let team = load_team(&state.pool, id).await?;
    if !access(&state.pool, &team, &user).await? {
        return Err(ApiError::forbidden(
            "only the game master can correct values",
        ));
    }
    let note = log::note(body.note)?;
    let mut tx = begin_write(&state.pool).await?;
    let characters = team_characters(&mut tx, id).await?;
    let mut character = characters
        .iter()
        .find(|c| c.id == character_id)
        .cloned()
        .ok_or_else(|| ApiError::not_found("character"))?;
    if let Some(base) = body.base {
        if !same_values(&base, &character.data.actor) {
            return Err(ApiError::conflict(
                "the character changed since the correction was opened",
            ));
        }
    }
    // Items change only through item commands, so a correction made with
    // an older copy never brings back what was handed over or used up.
    // Doses per Shift are the engine's count.
    let mut actor = body.actor;
    actor.items = character.data.actor.items.clone();
    actor.next_item = character.data.actor.next_item;
    actor.doses = character.data.actor.doses.clone();
    // Money changes only through bookings (F-76).
    actor.ledger = character.data.actor.ledger.clone();
    actor
        .supplies
        .retain(|s, _| matches!(s, Supply::Food | Supply::Water));
    characters::check_actor(&state, &actor)?;
    state.engine.check_overload(&character.data.actor, &actor)?;
    save_snapshot(&mut tx, &team, &characters).await?;
    let before = std::mem::replace(&mut character.data.actor, actor);
    characters::save(&mut tx, &character).await?;
    let changes = log::changes(&before, &character.data.actor);
    let mut entries = vec![Entry::Corrected {
        character: character.id,
        name: character.name.clone(),
    }];
    if !changes.is_empty() {
        entries.push(Entry::Changed {
            character: character.id,
            name: character.name.clone(),
            changes,
        });
    }
    let stored = log::append(&mut tx, id, &user, entries, note).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(stored))
}

/// The values a correction overwrites are the same; items, the count of
/// doses and the bookings are left out, since a correction keeps them
/// anyway.
fn same_values(base: &Actor, now: &Actor) -> bool {
    let mut base = base.clone();
    base.items.clone_from(&now.items);
    base.next_item = now.next_item;
    base.doses.clone_from(&now.doses);
    base.ledger.clone_from(&now.ledger);
    base == *now
}

/// An open decision as the interface shows it.
#[derive(Debug, Clone, Serialize)]
pub struct PendingView {
    pub id: u32,
    pub kind: &'static str,
    /// The character whose owner decides; none for the game master's
    /// choice of targets.
    pub actor: Option<String>,
    pub options: Vec<Choice>,
    pub question: Option<Id>,
    pub who: Option<Who>,
    pub candidates: Vec<String>,
    /// For a roll waiting for push or Story Point: the roll so far, so the
    /// decision can be made with the dice in view.
    pub roll: Option<RollView>,
    /// For a choice of targets: the character whose action or panic asks,
    /// the talent, panic effect or rule behind it, and what happens to
    /// each one chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Id>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ops: Vec<Op>,
}

/// A roll as far as it has gone; the engine counts the successes.
#[derive(Debug, Clone, Serialize)]
pub struct RollView {
    pub skill: Option<Id>,
    pub attribute: Option<Id>,
    pub base_dice: u8,
    pub stress_dice: u8,
    pub dice: Vec<Die>,
    pub successes: u8,
    pub pushes_left: u8,
}

impl From<&RollState> for RollView {
    fn from(roll: &RollState) -> Self {
        Self {
            skill: roll.skill.clone(),
            attribute: roll.attribute.clone(),
            base_dice: roll.base_dice,
            stress_dice: roll.stress_dice,
            dice: roll.dice.clone(),
            successes: roll.successes(),
            pushes_left: roll.max_pushes.saturating_sub(roll.pushes),
        }
    }
}

/// Open decisions; the choice of targets only for the game master. Whether
/// a player may spend a Story Point tells the others they have one, so
/// that option is for the owner and the game master only.
pub fn pending_views(state: &GameState, is_gm: bool, own: &[String]) -> Vec<PendingView> {
    state
        .pending
        .iter()
        .filter_map(|(id, pending)| {
            let mut view = PendingView {
                id: *id,
                kind: "",
                actor: decider(pending).map(str::to_owned),
                options: Vec::new(),
                question: None,
                who: None,
                candidates: Vec::new(),
                roll: None,
                by: None,
                source: None,
                ops: Vec::new(),
            };
            match pending {
                Pending::Roll(roll) => {
                    view.kind = "roll";
                    view.options = roll.offered.clone();
                    if !is_gm && !view.actor.as_ref().is_some_and(|a| own.contains(a)) {
                        view.options.retain(|c| *c != Choice::StoryPoint);
                    }
                    view.roll = Some(RollView::from(&**roll));
                }
                Pending::StartRoll { options, .. } => {
                    view.kind = "start_roll";
                    view.options = options.clone();
                }
                Pending::Overkill { .. } => {
                    view.kind = "overkill";
                    view.options = vec![Choice::Overkill, Choice::Accept];
                }
                Pending::Question { question, .. } => {
                    view.kind = "question";
                    view.question = Some(question.clone());
                    view.options = vec![Choice::Answer(true), Choice::Answer(false)];
                }
                Pending::Targets {
                    actor,
                    owner,
                    who,
                    candidates,
                    ops,
                } => {
                    if !is_gm {
                        return None;
                    }
                    view.kind = "targets";
                    view.who = Some(*who);
                    view.candidates = candidates.clone();
                    view.by = Some(actor.clone());
                    view.source = Some(owner.clone());
                    view.ops = ops.clone();
                }
            }
            Some(view)
        })
        .collect()
}

/// Drops open decisions about a character that leaves the team.
pub fn forget_actor(state: &mut GameState, actor: &str) {
    state.pending.retain(|_, p| match p {
        Pending::Targets { actor: a, .. } => a != actor,
        other => decider(other) != Some(actor),
    });
}
