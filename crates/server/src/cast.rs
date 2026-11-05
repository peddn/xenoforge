//! Who plays whom (A-12, A-13): the game master offers characters in the
//! team to choose, players take one, and the game master hands characters
//! over or takes them back. In Cinematic play the game master also sets
//! each character's Agenda per Act [p. 26, 31–32, 216].
//!
//! Only the owner changes; story points follow the owner as player, and
//! rights and visibility keep asking one person per character (plan 5.0).

use axum::extract::{Path, State};
use axum::routing::{post, put};
use axum::{Json, Router};
use serde::Deserialize;
use xenoforge_engine::state::Mode;

use crate::auth::User;
use crate::characters::{self, Character};
use crate::error::{ApiError, ApiResult};
use crate::game::{self, Team};
use crate::log::{self, Entry, Stored};
use crate::teams::name_of;
use crate::{AppState, begin_write};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{id}/characters/{character}/offer", put(offer))
        .route("/{id}/characters/{character}/claim", post(claim))
        .route("/{id}/characters/{character}/owner", put(owner))
        .route("/{id}/characters/{character}/agendas", put(agendas))
}

/// The team, whether the person is its game master, and a character of
/// it.
async fn team_and_character(
    state: &AppState,
    user: &User,
    id: i64,
    character_id: i64,
) -> ApiResult<(Team, bool, Character)> {
    let team = game::load_team(&state.pool, id).await?;
    let is_gm = game::access(&state.pool, &team, user).await?;
    let character = characters::load(&state.pool, character_id).await?;
    if character.team_id != Some(id) {
        return Err(ApiError::not_found("character"));
    }
    Ok((team, is_gm, character))
}

fn gm_only(is_gm: bool) -> ApiResult<()> {
    if is_gm {
        Ok(())
    } else {
        Err(ApiError::forbidden("only the game master can do that"))
    }
}

/// Saves the character and logs one entry. A new owner ends the undo
/// history, so undo never seems to take back a handover.
async fn save_and_log(
    state: &AppState,
    team: &Team,
    user: &User,
    character: &Character,
    entry: Entry,
) -> ApiResult<Vec<Stored>> {
    let mut tx = begin_write(&state.pool).await?;
    characters::save(&mut tx, character).await?;
    if matches!(
        entry,
        Entry::CharacterClaimed { .. } | Entry::CharacterHandedOver { .. }
    ) {
        sqlx::query("DELETE FROM snapshots WHERE team_id = ?")
            .bind(team.id)
            .execute(&mut *tx)
            .await?;
    }
    let stored = log::append(&mut tx, team.id, user, vec![entry], None).await?;
    tx.commit().await?;
    log::publish(state, &stored);
    Ok(stored)
}

#[derive(Deserialize)]
struct Offer {
    offered: bool,
}

/// The game master offers one of their own characters in the team, or
/// takes the offer back (A-12).
async fn offer(
    State(state): State<AppState>,
    user: User,
    Path((id, character_id)): Path<(i64, i64)>,
    Json(body): Json<Offer>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let (team, is_gm, mut character) = team_and_character(&state, &user, id, character_id).await?;
    gm_only(is_gm)?;
    if character.owner_id != team.gm_id {
        return Err(ApiError::conflict(
            "only the game master's own characters can be offered",
        ));
    }
    character.data.offered = body.offered;
    let entry = Entry::CharacterOffered {
        character: character.id,
        name: character.name.clone(),
        offered: body.offered,
    };
    Ok(Json(
        save_and_log(&state, &team, &user, &character, entry).await?,
    ))
}

/// A player takes an offered character and owns it at once; it is then no
/// longer on offer. Whoever comes first gets it; who already plays a
/// character in the team takes no second one (A-12, p. 26).
async fn claim(
    State(state): State<AppState>,
    user: User,
    Path((id, character_id)): Path<(i64, i64)>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let (team, is_gm, mut character) = team_and_character(&state, &user, id, character_id).await?;
    if is_gm {
        return Err(ApiError::forbidden(
            "the game master offers characters, players choose them",
        ));
    }
    if !character.data.offered || character.owner_id != team.gm_id {
        return Err(ApiError::conflict("the character is no longer on offer"));
    }
    let plays = sqlx::query("SELECT 1 FROM characters WHERE team_id = ? AND owner_id = ?")
        .bind(id)
        .bind(user.id)
        .fetch_optional(&state.pool)
        .await?
        .is_some();
    if plays {
        return Err(ApiError::conflict(
            "you already play a character in this team",
        ));
    }
    character.owner_id = user.id;
    character.data.offered = false;
    character.data.lent_by = Some(team.gm_id);
    let entry = Entry::CharacterClaimed {
        character: character.id,
        name: character.name.clone(),
    };
    Ok(Json(
        save_and_log(&state, &team, &user, &character, entry).await?,
    ))
}

#[derive(Deserialize)]
struct Owner {
    owner: i64,
}

/// The game master gives one of their characters to a member of the
/// team, e.g. as a replacement after death or PvP, or takes a prepared
/// character back. Taking it back is for Cinematic play only, where a
/// traitor becomes an NPC [p. 32]; characters players built stay theirs
/// (plan 5.0).
async fn owner(
    State(state): State<AppState>,
    user: User,
    Path((id, character_id)): Path<(i64, i64)>,
    Json(body): Json<Owner>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let (team, is_gm, mut character) = team_and_character(&state, &user, id, character_id).await?;
    gm_only(is_gm)?;
    if body.owner == character.owner_id {
        return Err(ApiError::conflict("the character already belongs to them"));
    }
    let member = sqlx::query("SELECT 1 FROM members WHERE team_id = ? AND user_id = ?")
        .bind(id)
        .bind(body.owner)
        .fetch_optional(&state.pool)
        .await?
        .is_some();
    if body.owner != team.gm_id && !member {
        return Err(ApiError::bad_request(
            "characters go to members of the team",
        ));
    }
    if character.owner_id != team.gm_id {
        if team.state.mode != Mode::Cinematic {
            return Err(ApiError::forbidden(
                "taking a character from a player is for Cinematic play only (p. 32)",
            ));
        }
        if body.owner != team.gm_id {
            return Err(ApiError::bad_request(
                "a player's character goes back to the game master only",
            ));
        }
        if character.data.status != characters::Status::Free {
            return Err(ApiError::forbidden(
                "only prepared characters go back to the game master",
            ));
        }
    }
    character.owner_id = body.owner;
    character.data.offered = false;
    character.data.lent_by = (body.owner != team.gm_id).then_some(team.gm_id);
    let entry = Entry::CharacterHandedOver {
        character: character.id,
        name: character.name.clone(),
        to: name_of(&state, body.owner).await?,
    };
    Ok(Json(
        save_and_log(&state, &team, &user, &character, entry).await?,
    ))
}

#[derive(Deserialize)]
struct Agendas {
    agendas: Vec<String>,
}

/// The game master sets a character's Agendas, one per Act, at most as
/// many as a scenario has Acts; the log says only that they changed
/// [p. 31, 216].
async fn agendas(
    State(state): State<AppState>,
    user: User,
    Path((id, character_id)): Path<(i64, i64)>,
    Json(body): Json<Agendas>,
) -> ApiResult<Json<Vec<Stored>>> {
    let _guard = state.lock.lock().await;
    let (mut team, is_gm, character) = team_and_character(&state, &user, id, character_id).await?;
    gm_only(is_gm)?;
    if team.state.mode != Mode::Cinematic {
        return Err(ApiError::bad_request(
            "Agendas per Act exist in Cinematic play only",
        ));
    }
    let acts = usize::from(state.engine.constant_u8("cinematic_acts"));
    if body.agendas.len() > acts {
        return Err(ApiError::bad_request(format!("a scenario has {acts} Acts")));
    }
    let texts: Vec<&str> = body.agendas.iter().map(String::as_str).collect();
    characters::check_texts(&texts)?;
    let mut list: Vec<String> = body.agendas.iter().map(|a| a.trim().to_owned()).collect();
    while list.last().is_some_and(String::is_empty) {
        list.pop();
    }
    if list.is_empty() {
        team.agendas.remove(&character.id);
    } else {
        team.agendas.insert(character.id, list);
    }
    let mut tx = begin_write(&state.pool).await?;
    game::save_agendas(&mut tx, &team).await?;
    let entry = Entry::AgendaChanged {
        character: character.id,
        name: character.name.clone(),
    };
    let stored = log::append(&mut tx, id, &user, vec![entry], None).await?;
    tx.commit().await?;
    log::publish(&state, &stored);
    Ok(Json(stored))
}
