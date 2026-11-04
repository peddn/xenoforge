//! Playing in a team: commands to the rule engine, previews, corrections
//! and undo.
//!
//! For every command the server puts the engine state together from the
//! team document and the team's characters, lets the engine work, and
//! writes everything back in one transaction. Each value is stored in one
//! place only.

use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use xenoforge_engine::data::Id;
use xenoforge_engine::data::effects::Who;
use xenoforge_engine::roll::RollRequest;
use xenoforge_engine::state::{Actor, GameState, Pending};
use xenoforge_engine::{Choice, Command, Preview};

use crate::auth::User;
use crate::characters::{self, Character, CharacterData};
use crate::error::{ApiError, ApiResult};
use crate::log::{self, Entry, Stored};
use crate::{AppState, now};

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

#[derive(Serialize, Deserialize)]
struct Snapshot {
    team: GameState,
    characters: Vec<(i64, CharacterData)>,
}

async fn save_snapshot(
    tx: &mut Transaction<'_, Sqlite>,
    team: &Team,
    characters: &[Character],
) -> ApiResult<()> {
    let snapshot = Snapshot {
        team: team.state.clone(),
        characters: characters.iter().map(|c| (c.id, c.data.clone())).collect(),
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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{id}/commands", post(command))
        .route("/{id}/preview", post(preview))
        .route("/{id}/undo", post(undo))
}

#[derive(Deserialize)]
struct CommandBody {
    command: Command,
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
    let mut tx = state.pool.begin().await?;
    let team = load_team_tx(&mut tx, id).await?;
    let mut characters = team_characters(&mut tx, id).await?;
    let owners: HashMap<String, i64> = characters
        .iter()
        .map(|c| (c.id.to_string(), c.owner_id))
        .collect();
    check_permission(&team, &body.command, &user, is_gm, &owners)?;

    let mut game = assemble(&team, &characters);
    let events = {
        let mut dice = state
            .dice
            .lock()
            .map_err(|_| ApiError::internal("dice lock"))?;
        state.engine.apply(&mut game, body.command, &mut **dice)?
    };
    save_snapshot(&mut tx, &team, &characters).await?;
    store(&mut tx, id, game, &mut characters).await?;
    let entries = events
        .into_iter()
        .map(|event| Entry::Engine { event })
        .collect();
    let stored = log::append(&mut tx, id, &user, entries).await?;
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
    let mut tx = state.pool.begin().await?;
    let row =
        sqlx::query("SELECT id, state FROM snapshots WHERE team_id = ? ORDER BY id DESC LIMIT 1")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| ApiError::conflict("nothing to undo"))?;
    let snapshot: Snapshot = serde_json::from_str(row.get("state"))?;
    save_team_state(&mut tx, id, &snapshot.team).await?;
    for character in team_characters(&mut tx, id).await? {
        if let Some((_, data)) = snapshot
            .characters
            .iter()
            .find(|(cid, _)| *cid == character.id)
        {
            let restored = Character {
                data: data.clone(),
                ..character
            };
            characters::save(&mut tx, &restored).await?;
        }
    }
    sqlx::query("DELETE FROM snapshots WHERE id = ?")
        .bind(row.get::<i64, _>("id"))
        .execute(&mut *tx)
        .await?;
    let stored = log::append(&mut tx, id, &user, vec![Entry::Undone]).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(stored))
}

#[derive(Deserialize)]
pub struct Correction {
    actor: Actor,
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
    let mut tx = state.pool.begin().await?;
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
    let stored = log::append(&mut tx, id, &user, vec![entry]).await?;
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
            };
            match pending {
                Pending::Roll(roll) => {
                    view.kind = "roll";
                    view.options = roll.offered.clone();
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
