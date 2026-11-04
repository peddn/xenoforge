//! Playing in a team: commands to the rule engine, previews, corrections
//! and undo.
//!
//! For every command the server puts the engine state together from the
//! team document and the team's characters, lets the engine work, and
//! writes everything back in one transaction. Each value is stored in one
//! place only.

use std::collections::HashMap;
use std::sync::PoisonError;

use axum::extract::{Path, State};
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use xenoforge_engine::data::Id;
use xenoforge_engine::data::effects::Who;
use xenoforge_engine::dice::Die;
use xenoforge_engine::roll::{RollKind, RollRequest, RollState};
use xenoforge_engine::state::{Actor, GameState, Pending};
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
}

fn team_from_row(row: &sqlx::sqlite::SqliteRow) -> ApiResult<Team> {
    let state: String = row.get("state");
    Ok(Team {
        id: row.get("id"),
        gm_id: row.get("gm_id"),
        name: row.get("name"),
        invite_token: row.get("invite_token"),
        state: serde_json::from_str(&state)?,
    })
}

pub async fn load_team(pool: &SqlitePool, id: i64) -> ApiResult<Team> {
    let row = sqlx::query("SELECT * FROM teams WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::not_found("team"))?;
    team_from_row(&row)
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
/// Story Points belong to the player, so each actor names its owner.
pub fn assemble(team: &Team, characters: &[Character]) -> GameState {
    let mut game = team.state.clone();
    game.actors = characters
        .iter()
        .map(|c| {
            let mut actor = c.data.actor.clone();
            actor.player = Some(c.owner_id.to_string());
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
/// answer their own decisions; everything else is the game master's.
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
        Command::UseSignatureItem { actor } | Command::PerformAction { actor, .. } => owns(actor),
        _ => false,
    };
    if allowed {
        Ok(())
    } else {
        Err(ApiError::forbidden("only the game master can do that"))
    }
}

/// What a roll may bring along (A-08): a difficulty from the table on
/// p. 63 and a gear bonus up to `gear_bonus_max`, each at most once. The
/// difficulty is the game master's; players roll skill, attribute and death
/// rolls only.
fn check_roll(engine: &Engine, request: &RollRequest, is_gm: bool) -> ApiResult<()> {
    let player_kind = matches!(
        request.kind,
        RollKind::Skill { .. } | RollKind::Attribute { .. } | RollKind::Death
    );
    if !is_gm && !player_kind {
        return Err(ApiError::forbidden(
            "players roll skill, attribute and death rolls (A-08)",
        ));
    }
    let (mut gear, mut difficulty) = (false, false);
    for modification in &request.modifiers {
        let value = i16::from(modification.value);
        if modification.source == "gear" {
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
    #[serde(default)]
    note: Option<String>,
}

/// The game master overwrites a character's values; logged and undoable
/// (plan section 3).
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
    characters::check_actor(&state, &body.actor)?;
    let note = log::note(body.note)?;
    let mut tx = begin_write(&state.pool).await?;
    let characters = team_characters(&mut tx, id).await?;
    let mut character = characters
        .iter()
        .find(|c| c.id == character_id)
        .cloned()
        .ok_or_else(|| ApiError::not_found("character"))?;
    save_snapshot(&mut tx, &team, &characters).await?;
    character.data.actor = body.actor;
    characters::save(&mut tx, &character).await?;
    let entry = Entry::Corrected {
        character: character.id,
        name: character.name.clone(),
    };
    let stored = log::append(&mut tx, id, &user, vec![entry], note).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(stored))
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

/// Open decisions; the choice of targets only for the game master.
pub fn pending_views(state: &GameState, is_gm: bool) -> Vec<PendingView> {
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
            };
            match pending {
                Pending::Roll(roll) => {
                    view.kind = "roll";
                    view.options = roll.offered.clone();
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
                    who, candidates, ..
                } => {
                    if !is_gm {
                        return None;
                    }
                    view.kind = "targets";
                    view.who = Some(*who);
                    view.candidates = candidates.clone();
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
