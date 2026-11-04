//! Characters belong to the person who created them (plan 5.0). A
//! character is one JSON document: the sheet plus the engine's actor.
//! Outside a team the owner edits it freely; inside a team its game values
//! change only through commands and corrections by the game master.

use axum::extract::{Path, State};
use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, Transaction};
use xenoforge_engine::state::Actor;

use crate::auth::User;
use crate::error::{ApiError, ApiResult};
use crate::{AppState, now};

/// Fields of the sheet that are not rules (phase 2b adds more).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sheet {
    pub appearance: String,
    pub notes: String,
}

/// Stored in `characters.data`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CharacterData {
    pub sheet: Sheet,
    pub actor: Actor,
}

#[derive(Debug, Clone, Serialize)]
pub struct Character {
    pub id: i64,
    pub owner_id: i64,
    pub team_id: Option<i64>,
    pub name: String,
    pub data: CharacterData,
}

#[derive(Deserialize)]
struct NewCharacter {
    name: String,
}

#[derive(Deserialize)]
struct Update {
    name: String,
    #[serde(default)]
    data: CharacterData,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(read).put(update).delete(remove))
}

pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> ApiResult<Character> {
    let data: String = row.get("data");
    Ok(Character {
        id: row.get("id"),
        owner_id: row.get("owner_id"),
        team_id: row.get("team_id"),
        name: row.get("name"),
        data: serde_json::from_str(&data)?,
    })
}

pub async fn load(pool: &sqlx::SqlitePool, id: i64) -> ApiResult<Character> {
    let row = sqlx::query("SELECT * FROM characters WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::not_found("character"))?;
    from_row(&row)
}

pub async fn save(tx: &mut Transaction<'_, Sqlite>, character: &Character) -> ApiResult<()> {
    sqlx::query("UPDATE characters SET name = ?, team_id = ?, data = ? WHERE id = ?")
        .bind(&character.name)
        .bind(character.team_id)
        .bind(serde_json::to_string(&character.data)?)
        .bind(character.id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

fn check_name(name: &str) -> ApiResult<String> {
    let name = name.trim();
    if (1..=60).contains(&name.chars().count()) {
        Ok(name.to_owned())
    } else {
        Err(ApiError::bad_request("the name needs 1 to 60 characters"))
    }
}

/// Ids in the actor must exist in the rule data.
pub fn check_actor(state: &AppState, actor: &Actor) -> ApiResult<()> {
    let rules = state.engine.rules();
    let unknown = |kind: &str, id: &str| ApiError::bad_request(format!("unknown {kind} \"{id}\""));
    for id in actor.attributes.keys() {
        if !rules.attributes.iter().any(|a| &a.id == id) {
            return Err(unknown("attribute", id));
        }
    }
    for id in actor.skills.keys() {
        if !rules.skills.iter().any(|s| &s.id == id) {
            return Err(unknown("skill", id));
        }
    }
    for id in &actor.talents {
        if !rules.talents.iter().any(|t| &t.id == id) {
            return Err(unknown("talent", id));
        }
    }
    Ok(())
}

async fn list(State(state): State<AppState>, user: User) -> ApiResult<Json<Vec<Character>>> {
    let rows = sqlx::query("SELECT * FROM characters WHERE owner_id = ? ORDER BY name")
        .bind(user.id)
        .fetch_all(&state.pool)
        .await?;
    Ok(Json(rows.iter().map(from_row).collect::<ApiResult<_>>()?))
}

async fn create(
    State(state): State<AppState>,
    user: User,
    Json(body): Json<NewCharacter>,
) -> ApiResult<Json<Character>> {
    let name = check_name(&body.name)?;
    let data = CharacterData {
        sheet: Sheet::default(),
        actor: Actor::pc(),
    };
    let id = sqlx::query(
        "INSERT INTO characters (owner_id, name, data, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(user.id)
    .bind(&name)
    .bind(serde_json::to_string(&data)?)
    .bind(now())
    .execute(&state.pool)
    .await?
    .last_insert_rowid();
    Ok(Json(Character {
        id,
        owner_id: user.id,
        team_id: None,
        name,
        data,
    }))
}

async fn owned(state: &AppState, user: &User, id: i64) -> ApiResult<Character> {
    let character = load(&state.pool, id).await?;
    if character.owner_id != user.id {
        return Err(ApiError::not_found("character"));
    }
    Ok(character)
}

async fn read(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> ApiResult<Json<Character>> {
    Ok(Json(owned(&state, &user, id).await?))
}

/// Name and sheet can always be edited. The values that count for the
/// rules only while the character is in no team (plan 5.0).
async fn update(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(body): Json<Update>,
) -> ApiResult<Json<Character>> {
    let mut character = owned(&state, &user, id).await?;
    character.name = check_name(&body.name)?;
    character.data.sheet = body.data.sheet;
    if character.team_id.is_none() {
        let new = body.data.actor;
        check_actor(&state, &new)?;
        let actor = &mut character.data.actor;
        // A character at full Health stays at full Health when STRENGTH
        // changes, unless Health itself was changed [p. 30].
        let was_full = actor.health == actor.full_health();
        let health_untouched = new.health == actor.health;
        actor.android = new.android;
        actor.attributes = new.attributes;
        actor.skills = new.skills;
        actor.talents = new.talents;
        actor.health = if was_full && health_untouched {
            actor.full_health()
        } else {
            new.health
        };
        actor.stress = new.stress;
    }
    let mut tx = state.pool.begin().await?;
    save(&mut tx, &character).await?;
    tx.commit().await?;
    Ok(Json(character))
}

async fn remove(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> ApiResult<Json<serde_json::Value>> {
    let character = owned(&state, &user, id).await?;
    if character.team_id.is_some() {
        return Err(ApiError::conflict("the character is in a team"));
    }
    sqlx::query("DELETE FROM characters WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({ "ok": true })))
}
