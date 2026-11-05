//! Characters belong to the person who created them (plan 5.0). A
//! character is one JSON document: status, the wizard's draft, the sheet
//! and the engine's actor.
//!
//! Players create characters with the wizard; the engine checks each draft
//! and finishing turns it into a playable character (A-09, plan 5.2). Only
//! game master accounts set up characters freely, and only those values can
//! be edited outside a team. Inside a team game values change only through
//! commands and corrections by the game master.

use std::sync::PoisonError;

use axum::extract::{Path, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, Transaction};
use xenoforge_engine::Command;
use xenoforge_engine::creation::{Cash, CreationProblem, Draft, GearRoll};
use xenoforge_engine::data::Id;
use xenoforge_engine::inventory::{Destination, Encumbrance, NewItem};
use xenoforge_engine::state::{Actor, ActorKind, GameState, Mode, Supply};

use crate::auth::{Role, User};
use crate::error::{ApiError, ApiResult};
use crate::{AppState, begin_write, now};

/// How a character came about (A-09).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// In the wizard; it cannot join a team yet.
    Draft,
    /// Created by the rules; its values change only in play.
    Finished,
    /// Set up freely by a game master. Characters from before the wizard
    /// count as free.
    #[default]
    Free,
}

/// Fields of the sheet that are not rules [p. 30–33, 389].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sheet {
    pub career: Option<Id>,
    pub appearance: String,
    /// Only the owner and the game master see it [p. 31].
    pub agenda: String,
    pub signature_item: String,
    /// Other player characters of the same team (F-52) [p. 31].
    pub buddy: Option<i64>,
    pub rival: Option<i64>,
    pub notes: String,
}

/// Stored in `characters.data`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CharacterData {
    pub status: Status,
    /// The wizard's state while the character is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
    pub sheet: Sheet,
    pub actor: Actor,
    /// Starting cash as rolled when finishing [p. 39–55].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cash: Option<Cash>,
    /// Quantities of starting gear rolled when finishing (F-60).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub start_gear: Vec<GearRoll>,
    /// Offered in its team for a player to choose (A-12).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub offered: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Character {
    pub id: i64,
    pub owner_id: i64,
    pub team_id: Option<i64>,
    pub name: String,
    pub data: CharacterData,
    /// What keeps a draft from being finished, from the engine.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub problems: Vec<CreationProblem>,
    /// How much the character carries, from the engine [p. 34].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encumbrance: Option<Encumbrance>,
}

impl Character {
    /// With what the engine says about it: the problems of a draft, the
    /// encumbrance.
    fn checked(mut self, state: &AppState) -> Self {
        if let (Status::Draft, Some(draft)) = (self.data.status, &self.data.draft) {
            self.problems = state.engine.check_creation(draft);
        }
        self.encumbrance = Some(state.engine.encumbrance(&self.data.actor));
        self
    }
}

/// A draft for the wizard, or with `free` a character set up freely by a
/// game master (A-09).
#[derive(Deserialize, Default)]
#[serde(default)]
struct NewCharacter {
    free: bool,
    name: String,
}

/// A draft takes only `data.draft`; its name is in there.
#[derive(Deserialize, Default)]
#[serde(default)]
struct Update {
    name: String,
    data: CharacterData,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(read).put(update).delete(remove))
        .route("/{id}/finish", post(finish))
        .route("/{id}/relations", put(relations))
        .route("/{id}/items", post(items))
}

pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> ApiResult<Character> {
    let data: String = row.get("data");
    Ok(Character {
        id: row.get("id"),
        owner_id: row.get("owner_id"),
        team_id: row.get("team_id"),
        name: row.get("name"),
        data: serde_json::from_str(&data)?,
        problems: Vec::new(),
        encumbrance: None,
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
    sqlx::query("UPDATE characters SET owner_id = ?, name = ?, team_id = ?, data = ? WHERE id = ?")
        .bind(character.owner_id)
        .bind(&character.name)
        .bind(character.team_id)
        .bind(serde_json::to_string(&character.data)?)
        .bind(character.id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// The longest name, in characters.
const NAME_MAX: usize = 60;

fn check_name(name: &str) -> ApiResult<String> {
    let name = name.trim();
    if (1..=NAME_MAX).contains(&name.chars().count()) {
        Ok(name.to_owned())
    } else {
        Err(ApiError::bad_request("the name needs 1 to 60 characters"))
    }
}

/// The longest sheet text, in characters.
pub const TEXT_MAX: usize = 4000;

pub fn check_texts(texts: &[&str]) -> ApiResult<()> {
    if texts.iter().any(|t| t.chars().count() > TEXT_MAX) {
        return Err(ApiError::bad_request(format!(
            "a text has at most {TEXT_MAX} characters"
        )));
    }
    Ok(())
}

/// A draft may have no name yet.
fn draft_name(name: &str) -> ApiResult<String> {
    let name = name.trim();
    if name.chars().count() <= NAME_MAX {
        Ok(name.to_owned())
    } else {
        Err(ApiError::bad_request("the name has at most 60 characters"))
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
    for item in &actor.items {
        check_texts(&[item.name.as_deref().unwrap_or_default()])?;
    }
    state.engine.check_items(actor)?;
    Ok(())
}

async fn list(State(state): State<AppState>, user: User) -> ApiResult<Json<Vec<Character>>> {
    let rows = sqlx::query("SELECT * FROM characters WHERE owner_id = ? ORDER BY name")
        .bind(user.id)
        .fetch_all(&state.pool)
        .await?;
    let list = rows
        .iter()
        .map(|row| Ok(from_row(row)?.checked(&state)))
        .collect::<ApiResult<_>>()?;
    Ok(Json(list))
}

async fn create(
    State(state): State<AppState>,
    user: User,
    Json(body): Json<NewCharacter>,
) -> ApiResult<Json<Character>> {
    let (name, data) = if body.free {
        if user.role != Role::Gm {
            return Err(ApiError::forbidden(
                "only game master accounts set up characters freely (A-09)",
            ));
        }
        let data = CharacterData {
            status: Status::Free,
            actor: Actor::pc(),
            ..CharacterData::default()
        };
        (check_name(&body.name)?, data)
    } else {
        let draft = Draft::default();
        let data = CharacterData {
            status: Status::Draft,
            actor: state.engine.creation_actor(&draft),
            draft: Some(draft),
            ..CharacterData::default()
        };
        (String::new(), data)
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
    let character = Character {
        id,
        owner_id: user.id,
        team_id: None,
        name,
        data,
        problems: Vec::new(),
        encumbrance: None,
    };
    Ok(Json(character.checked(&state)))
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
    Ok(Json(owned(&state, &user, id).await?.checked(&state)))
}

/// A draft takes the wizard's state; the engine builds the actor from it
/// and names the problems. Otherwise name and sheet texts can always be
/// edited, the agenda too (F-54); buddy and rival have their own route.
/// The values that count for the rules only on a free character, by a
/// game master account, outside a team (A-09, plan 5.0).
async fn update(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(body): Json<Update>,
) -> ApiResult<Json<Character>> {
    // Under the lock: a command in the team must not be overwritten with
    // values read before it.
    let _guard = state.lock.lock().await;
    let mut character = owned(&state, &user, id).await?;
    if character.data.status == Status::Draft {
        let draft = body
            .data
            .draft
            .ok_or_else(|| ApiError::bad_request("a draft is saved as data.draft"))?;
        check_texts(&[
            &draft.appearance,
            &draft.agenda,
            &draft.signature_item,
            draft.talent_param.as_deref().unwrap_or_default(),
        ])?;
        character.name = draft_name(&draft.name)?;
        character.data.actor = state.engine.creation_actor(&draft);
        character.data.draft = Some(draft);
        let mut tx = begin_write(&state.pool).await?;
        save(&mut tx, &character).await?;
        tx.commit().await?;
        return Ok(Json(character.checked(&state)));
    }
    let new = body.data.sheet;
    check_texts(&[
        &new.appearance,
        &new.agenda,
        &new.signature_item,
        &new.notes,
    ])?;
    character.name = check_name(&body.name)?;
    let free = character.data.status == Status::Free && user.role == Role::Gm;
    let sheet = &mut character.data.sheet;
    sheet.appearance = new.appearance;
    sheet.agenda = new.agenda;
    sheet.signature_item = new.signature_item;
    sheet.notes = new.notes;
    if free {
        if let Some(career) = &new.career {
            if !state.engine.rules().careers.iter().any(|c| &c.id == career) {
                return Err(ApiError::bad_request(format!(
                    "unknown career \"{career}\""
                )));
            }
        }
        sheet.career = new.career;
    }
    if free && character.team_id.is_none() {
        let new = body.data.actor;
        let before = character.data.actor.clone();
        let actor = &mut character.data.actor;
        // A character at full Health stays at full Health when STRENGTH
        // changes, unless Health itself was changed [p. 30].
        let was_full = actor.health == state.engine.full_health(actor);
        let health_untouched = new.health == actor.health;
        actor.android = new.android;
        actor.covert = new.android && new.covert;
        actor.attributes = new.attributes;
        actor.skills = new.skills;
        actor.talents = new.talents;
        actor.health = if was_full && health_untouched {
            state.engine.full_health(actor)
        } else {
            new.health
        };
        actor.stress = new.stress;
        actor.talent_params = new.talent_params;
        // Food and Water; air and power are in the items (F-75).
        actor.supplies = new
            .supplies
            .into_iter()
            .filter(|(s, _)| matches!(s, Supply::Food | Supply::Water))
            .collect();
        // The stored items with the new values, e.g. a lower STRENGTH.
        check_actor(&state, actor)?;
        state.engine.check_overload(&before, actor)?;
    }
    let mut tx = begin_write(&state.pool).await?;
    save(&mut tx, &character).await?;
    tx.commit().await?;
    Ok(Json(character.checked(&state)))
}

/// An item command for a character outside a team; in a team the same
/// commands go to the team.
#[derive(Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
enum ItemCommand {
    AddItem {
        item: NewItem,
    },
    MoveItem {
        uid: u32,
        to: Destination,
        #[serde(default)]
        count: Option<u16>,
    },
}

/// Gear of a free character outside a team, set up by its game master
/// (A-09): the engine's item commands on the character alone, so the same
/// rules hold as in play [p. 33, 34].
async fn items(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(body): Json<ItemCommand>,
) -> ApiResult<Json<Character>> {
    let _guard = state.lock.lock().await;
    let mut character = owned(&state, &user, id).await?;
    let free = character.data.status == Status::Free && user.role == Role::Gm;
    if !free || character.team_id.is_some() {
        return Err(ApiError::forbidden(
            "outside a team only free characters of a game master get items (A-09)",
        ));
    }
    let key = character.id.to_string();
    let command = match body {
        ItemCommand::AddItem { item } => {
            check_texts(&[item.name.as_deref().unwrap_or_default()])?;
            Command::AddItem {
                actor: key.clone(),
                item,
            }
        }
        ItemCommand::MoveItem {
            to: Destination::Character(_),
            ..
        } => {
            return Err(ApiError::bad_request("items are handed over in a team"));
        }
        ItemCommand::MoveItem { uid, to, count } => Command::MoveItem {
            actor: key.clone(),
            uid,
            to,
            count,
        },
    };
    let mut game = GameState::new(Mode::Campaign);
    game.actors
        .insert(key.clone(), character.data.actor.clone());
    {
        // Item commands throw no dice; the server's dice anyway.
        let mut dice = state.dice.lock().unwrap_or_else(PoisonError::into_inner);
        state.engine.apply(&mut game, command, &mut **dice)?;
    }
    if let Some(actor) = game.actors.remove(&key) {
        character.data.actor = actor;
    }
    let mut tx = begin_write(&state.pool).await?;
    save(&mut tx, &character).await?;
    tx.commit().await?;
    Ok(Json(character.checked(&state)))
}

#[derive(Deserialize)]
struct Relations {
    buddy: Option<i64>,
    rival: Option<i64>,
}

/// Buddy and rival: other player characters of the same team, not the
/// same one; both are optional (F-52) [p. 31]. In Campaign play the owner
/// chooses them, in Cinematic play the scenario does, so the game master
/// sets them [p. 31]. A route of their own, so saving texts never touches
/// them.
async fn relations(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(body): Json<Relations>,
) -> ApiResult<Json<Character>> {
    let _guard = state.lock.lock().await;
    let mut character = load(&state.pool, id).await?;
    let team = match character.team_id {
        Some(team) => Some(crate::game::load_team(&state.pool, team).await?),
        None => None,
    };
    let chooses = match &team {
        Some(team) if team.state.mode == Mode::Cinematic => team.gm_id == user.id,
        _ => character.owner_id == user.id,
    };
    if !chooses {
        return Err(if character.owner_id == user.id {
            ApiError::forbidden("in Cinematic play the scenario sets buddy and rival (p. 31)")
        } else {
            ApiError::not_found("character")
        });
    }
    check_relations(&state, &character, body.buddy, body.rival).await?;
    character.data.sheet.buddy = body.buddy;
    character.data.sheet.rival = body.rival;
    let mut tx = begin_write(&state.pool).await?;
    save(&mut tx, &character).await?;
    tx.commit().await?;
    Ok(Json(character.checked(&state)))
}

async fn check_relations(
    state: &AppState,
    character: &Character,
    buddy: Option<i64>,
    rival: Option<i64>,
) -> ApiResult<()> {
    if (buddy, rival) == (None, None) {
        return Ok(());
    }
    let Some(team) = character.team_id else {
        return Err(ApiError::bad_request(
            "buddy and rival are chosen within a team",
        ));
    };
    if buddy.is_some() && buddy == rival {
        return Err(ApiError::bad_request(
            "buddy and rival are two different characters (F-52)",
        ));
    }
    for other in [buddy, rival].into_iter().flatten() {
        let row = sqlx::query("SELECT * FROM characters WHERE id = ? AND team_id = ?")
            .bind(other)
            .bind(team)
            .fetch_optional(&state.pool)
            .await?;
        let pc = match row {
            Some(row) => from_row(&row)?.data.actor.kind == ActorKind::Pc,
            None => false,
        };
        if other == character.id || !pc {
            return Err(ApiError::bad_request(
                "buddy and rival are other player characters of the team (F-52)",
            ));
        }
    }
    Ok(())
}

/// Finishes a draft without problems: the actor with full Health and its
/// starting gear, the sheet texts, and starting cash and quantities of
/// gear rolled with the server's dice (F-60) [p. 27–39].
async fn finish(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> ApiResult<Json<Character>> {
    let _guard = state.lock.lock().await;
    let mut character = owned(&state, &user, id).await?;
    let draft = match (character.data.status, &character.data.draft) {
        (Status::Draft, Some(draft)) => draft.clone(),
        _ => return Err(ApiError::conflict("the character is not a draft")),
    };
    if !state.engine.check_creation(&draft).is_empty() {
        return Err(ApiError::bad_request("the draft is not complete yet"));
    }
    let career = draft.career.clone().unwrap_or_default();
    let mut actor = state.engine.creation_actor(&draft);
    let (start_cash, start_gear) = {
        // The dice hold no state worth protecting (see game.rs).
        let mut dice = state.dice.lock().unwrap_or_else(PoisonError::into_inner);
        (
            state.engine.roll_cash(&career, &mut **dice),
            state
                .engine
                .roll_start_gear(&mut actor, &draft, &mut **dice),
        )
    };
    // The starting cash is the first booking [p. 33].
    if let Some(cash) = &start_cash {
        xenoforge_engine::money::open_account(&mut actor, cash);
    }
    character.name = check_name(&draft.name)?;
    character.data = CharacterData {
        status: Status::Finished,
        draft: None,
        sheet: Sheet {
            career: draft.career.clone(),
            appearance: draft.appearance.trim().to_owned(),
            agenda: draft.agenda.trim().to_owned(),
            signature_item: draft.signature_item.trim().to_owned(),
            ..Sheet::default()
        },
        actor,
        start_cash,
        start_gear,
        offered: false,
    };
    let mut tx = begin_write(&state.pool).await?;
    save(&mut tx, &character).await?;
    tx.commit().await?;
    Ok(Json(character.checked(&state)))
}

async fn remove(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> ApiResult<Json<serde_json::Value>> {
    let _guard = state.lock.lock().await;
    let character = owned(&state, &user, id).await?;
    let deleted = sqlx::query("DELETE FROM characters WHERE id = ? AND team_id IS NULL")
        .bind(character.id)
        .execute(&state.pool)
        .await?
        .rows_affected();
    if deleted == 0 {
        return Err(ApiError::conflict("the character is in a team"));
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}
