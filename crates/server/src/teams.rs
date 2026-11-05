//! Teams (A-04, A-07): only game master accounts create them; players join
//! with an invitation link and bring one of their free characters.

use std::collections::BTreeMap;

use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use xenoforge_engine::state::{Actor, GameState, Mode};

use crate::auth::{Role, User};
use crate::characters::{self, Character};
use crate::error::{ApiError, ApiResult};
use crate::game::{self, PendingView, Team};
use crate::log::{self, Entry, Stored};
use crate::{AppState, begin_write, now, random_token};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(view))
        .route("/{id}/invite", post(new_invite))
        .route("/{id}/characters", post(add_own))
        .route(
            "/{id}/characters/{character}",
            delete(remove).put(game::correct),
        )
        .route("/{id}/members/{user}", delete(remove_member))
}

pub fn join_routes() -> Router<AppState> {
    Router::new().route("/{token}", get(join_info).post(join))
}

#[derive(Deserialize)]
struct NewTeam {
    name: String,
    mode: Mode,
}

#[derive(Serialize)]
struct TeamSummary {
    id: i64,
    name: String,
    mode: Mode,
    is_gm: bool,
}

async fn list(State(state): State<AppState>, user: User) -> ApiResult<Json<Vec<TeamSummary>>> {
    let rows = sqlx::query(
        "SELECT DISTINCT teams.* FROM teams LEFT JOIN members ON members.team_id = teams.id \
         WHERE teams.gm_id = ? OR members.user_id = ? ORDER BY teams.name",
    )
    .bind(user.id)
    .bind(user.id)
    .fetch_all(&state.pool)
    .await?;
    let mut teams = Vec::new();
    for row in rows {
        let state_json: String = row.get("state");
        let game: GameState = serde_json::from_str(&state_json)?;
        let gm_id: i64 = row.get("gm_id");
        teams.push(TeamSummary {
            id: row.get("id"),
            name: row.get("name"),
            mode: game.mode,
            is_gm: gm_id == user.id,
        });
    }
    Ok(Json(teams))
}

async fn create(
    State(state): State<AppState>,
    user: User,
    Json(body): Json<NewTeam>,
) -> ApiResult<Json<TeamSummary>> {
    if user.role != Role::Gm {
        return Err(ApiError::forbidden(
            "only game master accounts create teams (A-07)",
        ));
    }
    let name = body.name.trim().to_owned();
    if !(1..=60).contains(&name.chars().count()) {
        return Err(ApiError::bad_request("the name needs 1 to 60 characters"));
    }
    let id = sqlx::query(
        "INSERT INTO teams (gm_id, name, invite_token, state, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(user.id)
    .bind(&name)
    .bind(random_token()?)
    .bind(serde_json::to_string(&GameState::new(body.mode))?)
    .bind(now())
    .execute(&state.pool)
    .await?
    .last_insert_rowid();
    Ok(Json(TeamSummary {
        id,
        name,
        mode: body.mode,
        is_gm: true,
    }))
}

#[derive(Serialize)]
struct Person {
    id: i64,
    name: String,
}

#[derive(Serialize)]
struct CharacterInTeam {
    id: i64,
    name: String,
    owner_id: i64,
    owner_name: String,
    /// Values only for the game master and the owner (A-01), and for
    /// everyone while it is offered to choose (A-12).
    actor: Option<Actor>,
    /// Career, buddy and rival and the other texts, for the same people;
    /// the Campaign agenda only for the game master and the owner [p. 31].
    sheet: Option<characters::Sheet>,
    /// Offered to choose (A-12).
    offered: bool,
    /// The Agenda of the current Act, for the game master and the owner
    /// (A-13).
    agenda: Option<String>,
    /// All Agendas per Act, for the game master.
    agendas: Option<Vec<String>>,
    /// Broken as the engine sees it [p. 98], for the same people.
    broken: Option<bool>,
    /// Full Health [p. 30], for the same people.
    health_max: Option<u8>,
    /// What the character carries [p. 34], for the same people.
    encumbrance: Option<xenoforge_engine::inventory::Encumbrance>,
    /// Hints working on the character: (source, text key).
    hints: Vec<(String, String)>,
}

#[derive(Serialize)]
struct TeamView {
    id: i64,
    name: String,
    mode: Mode,
    gm: Person,
    is_gm: bool,
    /// The current Act in Cinematic play, counted from 1 [p. 216].
    act: Option<u8>,
    /// Only for the game master.
    invite_token: Option<String>,
    members: Vec<Person>,
    characters: Vec<CharacterInTeam>,
    pending: Vec<PendingView>,
    /// By user id; players see their own.
    story_points: BTreeMap<String, u8>,
    rulings: BTreeMap<String, String>,
}

pub async fn name_of(state: &AppState, user_id: i64) -> ApiResult<String> {
    Ok(sqlx::query("SELECT name FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?
        .get("name"))
}

async fn view(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> ApiResult<Json<TeamView>> {
    let team = game::load_team(&state.pool, id).await?;
    let is_gm = game::access(&state.pool, &team, &user).await?;
    let mut tx = state.pool.begin().await?;
    let characters = game::team_characters(&mut tx, id).await?;
    let members = sqlx::query(
        "SELECT users.id, users.name FROM members JOIN users ON users.id = members.user_id \
         WHERE members.team_id = ? ORDER BY users.name",
    )
    .bind(id)
    .fetch_all(&mut *tx)
    .await?;
    tx.rollback().await?;
    let game_state = game::assemble(&team, &characters);
    let mut in_team = Vec::new();
    let cinematic = team.state.mode == Mode::Cinematic;
    for character in &characters {
        let own = is_gm || character.owner_id == user.id;
        let sees = own || character.data.offered;
        let hints = if sees {
            state
                .engine
                .hints(&game_state, &character.id.to_string())
                .into_iter()
                .filter(|(_, _, gm_only)| is_gm || !gm_only)
                .map(|(label, text, _)| (label, text))
                .collect()
        } else {
            Vec::new()
        };
        in_team.push(CharacterInTeam {
            id: character.id,
            name: character.name.clone(),
            owner_id: character.owner_id,
            owner_name: name_of(&state, character.owner_id).await?,
            // An offered sheet for the others shows no agenda and no
            // notes, and a covert android passes as human (A-12, A-14,
            // p. 29).
            actor: sees.then(|| {
                let mut actor = character.data.actor.clone();
                if !own && actor.covert {
                    actor.android = false;
                    actor.covert = false;
                }
                // Money only for the owner and the game master (A-15).
                if !own {
                    actor.ledger.clear();
                }
                actor
            }),
            sheet: sees.then(|| {
                let mut sheet = character.data.sheet.clone();
                if !own {
                    sheet.agenda.clear();
                    sheet.notes.clear();
                }
                sheet
            }),
            offered: character.data.offered,
            agenda: (own && cinematic)
                .then(|| team.agenda_now(character.id))
                .flatten(),
            agendas: (is_gm && cinematic)
                .then(|| team.agendas.get(&character.id).cloned().unwrap_or_default()),
            broken: sees.then(|| character.data.actor.broken()),
            health_max: sees.then(|| state.engine.full_health(&character.data.actor)),
            encumbrance: sees.then(|| state.engine.encumbrance(&character.data.actor)),
            hints,
        });
    }
    let own = user.id.to_string();
    Ok(Json(TeamView {
        id,
        name: team.name.clone(),
        mode: team.state.mode,
        gm: Person {
            id: team.gm_id,
            name: name_of(&state, team.gm_id).await?,
        },
        is_gm,
        act: cinematic.then(|| team.state.acts_ended.saturating_add(1)),
        invite_token: is_gm.then(|| team.invite_token.clone()),
        members: members
            .iter()
            .map(|r| Person {
                id: r.get("id"),
                name: r.get("name"),
            })
            .collect(),
        characters: in_team,
        pending: game::pending_views(&team.state, is_gm),
        story_points: team
            .state
            .story_points
            .iter()
            .filter(|(player, _)| is_gm || **player == own)
            .map(|(k, v)| (k.clone(), *v))
            .collect(),
        rulings: team.state.rulings.clone().into_iter().collect(),
    }))
}

async fn gm_team(state: &AppState, user: &User, id: i64) -> ApiResult<Team> {
    let team = game::load_team(&state.pool, id).await?;
    if !game::access(&state.pool, &team, user).await? {
        return Err(ApiError::forbidden("only the game master can do that"));
    }
    Ok(team)
}

async fn new_invite(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> ApiResult<Json<serde_json::Value>> {
    gm_team(&state, &user, id).await?;
    let token = random_token()?;
    sqlx::query("UPDATE teams SET invite_token = ? WHERE id = ?")
        .bind(&token)
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({ "invite_token": token })))
}

#[derive(Deserialize)]
struct Bring {
    character: i64,
}

/// Puts a free character of `user` into the team and logs it.
async fn bring(
    state: &AppState,
    team: &Team,
    user: &User,
    character_id: i64,
) -> ApiResult<Vec<Stored>> {
    let _guard = state.lock.lock().await;
    let mut character: Character = characters::load(&state.pool, character_id).await?;
    if character.owner_id != user.id {
        return Err(ApiError::not_found("character"));
    }
    if character.team_id.is_some() {
        return Err(ApiError::conflict(
            "the character is already in a team (A-04)",
        ));
    }
    if character.data.status == characters::Status::Draft {
        return Err(ApiError::conflict(
            "a draft joins a team only when it is finished (A-09)",
        ));
    }
    let mut tx = begin_write(&state.pool).await?;
    character.team_id = Some(team.id);
    characters::save(&mut tx, &character).await?;
    if team.gm_id != user.id {
        sqlx::query("INSERT OR IGNORE INTO members (team_id, user_id) VALUES (?, ?)")
            .bind(team.id)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
    }
    let entry = Entry::CharacterJoined {
        character: character.id,
        name: character.name.clone(),
    };
    let stored = log::append(&mut tx, team.id, user, vec![entry], None).await?;
    tx.commit().await?;
    log::publish(state, &stored);
    Ok(stored)
}

/// The game master adds one of their own characters, e.g. a prepared one.
async fn add_own(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(body): Json<Bring>,
) -> ApiResult<Json<Vec<Stored>>> {
    let team = gm_team(&state, &user, id).await?;
    Ok(Json(bring(&state, &team, &user, body.character).await?))
}

/// The game master takes a character out of the team, or a player takes
/// their own character out and so leaves (A-04).
async fn remove(
    State(state): State<AppState>,
    user: User,
    Path((id, character_id)): Path<(i64, i64)>,
) -> ApiResult<Json<Vec<Stored>>> {
    // Lock first: the team document must not change between reading and
    // writing it back.
    let _guard = state.lock.lock().await;
    let mut team = game::load_team(&state.pool, id).await?;
    let is_gm = game::access(&state.pool, &team, &user).await?;
    let mut character = characters::load(&state.pool, character_id).await?;
    if character.team_id != Some(id) {
        return Err(ApiError::not_found("character"));
    }
    if !is_gm && character.owner_id != user.id {
        return Err(ApiError::forbidden(
            "only the game master or the owner can do that",
        ));
    }
    let mut tx = begin_write(&state.pool).await?;
    character.team_id = None;
    character.data.offered = false;
    if team.agendas.remove(&character.id).is_some() {
        game::save_agendas(&mut tx, &team).await?;
    }
    // Buddy and rival exist only within the team (F-52).
    character.data.sheet.buddy = None;
    character.data.sheet.rival = None;
    characters::save(&mut tx, &character).await?;
    for mut other in game::team_characters(&mut tx, id).await? {
        let sheet = &mut other.data.sheet;
        let before = (sheet.buddy, sheet.rival);
        sheet.buddy = sheet.buddy.filter(|c| *c != character.id);
        sheet.rival = sheet.rival.filter(|c| *c != character.id);
        if (sheet.buddy, sheet.rival) != before {
            characters::save(&mut tx, &other).await?;
        }
    }
    game::forget_actor(&mut team.state, &character_id.to_string());
    sqlx::query("UPDATE teams SET state = ? WHERE id = ?")
        .bind(serde_json::to_string(&team.state)?)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    // A player without a character in the team no longer sees it.
    sqlx::query(
        "DELETE FROM members WHERE team_id = ? AND user_id = ? AND NOT EXISTS \
         (SELECT 1 FROM characters WHERE team_id = ? AND owner_id = ?)",
    )
    .bind(id)
    .bind(character.owner_id)
    .bind(id)
    .bind(character.owner_id)
    .execute(&mut *tx)
    .await?;
    // Older states still refer to the character; undo stops here.
    sqlx::query("DELETE FROM snapshots WHERE team_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    let entry = if is_gm {
        Entry::CharacterRemoved {
            character: character.id,
            name: character.name.clone(),
        }
    } else {
        Entry::CharacterLeft {
            character: character.id,
            name: character.name.clone(),
        }
    };
    let stored = log::append(&mut tx, id, &user, vec![entry], None).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(stored))
}

/// A player without a character in the team leaves it, or the game master
/// takes them out (A-12). Whoever still plays a character leaves with it.
async fn remove_member(
    State(state): State<AppState>,
    user: User,
    Path((id, member)): Path<(i64, i64)>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let team = game::load_team(&state.pool, id).await?;
    let is_gm = game::access(&state.pool, &team, &user).await?;
    if !is_gm && member != user.id {
        return Err(ApiError::forbidden(
            "only the game master or the player themselves can do that",
        ));
    }
    let plays = sqlx::query("SELECT 1 FROM characters WHERE team_id = ? AND owner_id = ?")
        .bind(id)
        .bind(member)
        .fetch_optional(&state.pool)
        .await?
        .is_some();
    if plays {
        return Err(ApiError::conflict(
            "the player still has a character in the team",
        ));
    }
    let mut tx = begin_write(&state.pool).await?;
    let removed = sqlx::query("DELETE FROM members WHERE team_id = ? AND user_id = ?")
        .bind(id)
        .bind(member)
        .execute(&mut *tx)
        .await?
        .rows_affected();
    if removed == 0 {
        return Err(ApiError::not_found("member"));
    }
    let name = name_of(&state, member).await?;
    let entry = if member == user.id {
        Entry::MemberLeft { name }
    } else {
        Entry::MemberRemoved { name }
    };
    let stored = log::append(&mut tx, id, &user, vec![entry], None).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(stored))
}

#[derive(Serialize)]
struct JoinInfo {
    team: String,
    gm: String,
    mode: Mode,
}

async fn team_by_token(state: &AppState, token: &str) -> ApiResult<Team> {
    let id: i64 = sqlx::query("SELECT id FROM teams WHERE invite_token = ?")
        .bind(token)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| ApiError::not_found("invitation"))?
        .get("id");
    game::load_team(&state.pool, id).await
}

async fn join_info(
    State(state): State<AppState>,
    _user: User,
    Path(token): Path<String>,
) -> ApiResult<Json<JoinInfo>> {
    let team = team_by_token(&state, &token).await?;
    Ok(Json(JoinInfo {
        team: team.name.clone(),
        gm: name_of(&state, team.gm_id).await?,
        mode: team.state.mode,
    }))
}

#[derive(Deserialize)]
struct Join {
    character: Option<i64>,
}

/// A player joins with one of their free characters, or without one to
/// choose a character the game master offers (A-12).
async fn join(
    State(state): State<AppState>,
    user: User,
    Path(token): Path<String>,
    Json(body): Json<Join>,
) -> ApiResult<Json<serde_json::Value>> {
    let team = team_by_token(&state, &token).await?;
    match body.character {
        Some(character) => {
            bring(&state, &team, &user, character).await?;
        }
        None if team.gm_id != user.id => {
            let _guard = state.lock.lock().await;
            let mut tx = begin_write(&state.pool).await?;
            let added =
                sqlx::query("INSERT OR IGNORE INTO members (team_id, user_id) VALUES (?, ?)")
                    .bind(team.id)
                    .bind(user.id)
                    .execute(&mut *tx)
                    .await?
                    .rows_affected();
            let stored = if added > 0 {
                let entry = Entry::MemberJoined {
                    name: user.name.clone(),
                };
                log::append(&mut tx, team.id, &user, vec![entry], None).await?
            } else {
                Vec::new()
            };
            tx.commit().await?;
            log::publish(&state, &stored);
        }
        None => {}
    }
    Ok(Json(serde_json::json!({ "team": team.id })))
}
