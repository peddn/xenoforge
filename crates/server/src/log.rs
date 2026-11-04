//! The shared log and the event stream (plan 11.3).
//!
//! Every engine event and every change by hand becomes a log entry. Each
//! entry is visible to everyone in the team or only to the game master;
//! the filter is here and nowhere else. Clients follow the log as
//! Server-Sent Events; after a reconnect the browser sends the last id it
//! saw (`Last-Event-ID`) and gets what it missed.

use std::convert::Infallible;

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use axum::{Router, routing::get};
use futures_util::{Stream, StreamExt, stream};
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite, Transaction};
use tokio_stream::wrappers::BroadcastStream;
use xenoforge_engine::Event;

use crate::auth::User;
use crate::error::{ApiError, ApiResult};
use crate::game;
use crate::{AppState, now};

/// What a log entry says.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Entry {
    /// Something the rule engine did.
    Engine {
        event: Event,
    },
    CharacterJoined {
        character: i64,
        name: String,
    },
    /// The game master took the character out of the team.
    CharacterRemoved {
        character: i64,
        name: String,
    },
    /// The player took their character out of the team.
    CharacterLeft {
        character: i64,
        name: String,
    },
    /// The game master changed values by hand.
    Corrected {
        character: i64,
        name: String,
    },
    /// The game master took back the last step.
    Undone,
}

/// A stored log entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stored {
    pub id: i64,
    /// The command it came from: the id of that command's first entry.
    /// The interface shows each command as one block.
    pub batch: i64,
    #[serde(skip)]
    pub team_id: i64,
    /// `all` or `gm`.
    pub visible_to: String,
    /// Name of the person whose command caused it.
    pub by: Option<String>,
    /// Why the command was given, in the person's words; on the first
    /// entry of the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub at: i64,
    pub entry: Entry,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    by: Option<String>,
    /// Missing on a command's first entry, which is its own batch; older
    /// entries have none either and stand alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    batch: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    entry: Entry,
}

/// How long a note may be, in characters.
const NOTE_MAX: usize = 200;

/// A note as sent: trimmed, empty means none, at most `NOTE_MAX` characters.
pub fn note(text: Option<String>) -> ApiResult<Option<String>> {
    let text = text.map(|t| t.trim().to_owned()).filter(|t| !t.is_empty());
    if text.as_ref().is_some_and(|t| t.chars().count() > NOTE_MAX) {
        return Err(ApiError::bad_request(format!(
            "a note has at most {NOTE_MAX} characters"
        )));
    }
    Ok(text)
}

/// Hints for the game master only stay with the game master; the rest is
/// shared (A-01).
fn visibility(entry: &Entry) -> &'static str {
    match entry {
        Entry::Engine {
            event: Event::Hint { gm_only: true, .. },
        } => "gm",
        _ => "all",
    }
}

pub fn visible(stored: &Stored, is_gm: bool) -> bool {
    is_gm || stored.visible_to == "all"
}

/// Writes entries in the open transaction; the note goes with the first.
pub async fn append(
    tx: &mut Transaction<'_, Sqlite>,
    team_id: i64,
    user: &User,
    entries: Vec<Entry>,
    mut note: Option<String>,
) -> ApiResult<Vec<Stored>> {
    let mut stored: Vec<Stored> = Vec::with_capacity(entries.len());
    let at = now();
    for entry in entries {
        let visible_to = visibility(&entry).to_owned();
        let payload = Payload {
            by: Some(user.name.clone()),
            batch: stored.first().map(|first| first.id),
            note: note.take(),
            entry,
        };
        let id = sqlx::query(
            "INSERT INTO events (team_id, user_id, visible_to, payload, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(team_id)
        .bind(user.id)
        .bind(&visible_to)
        .bind(serde_json::to_string(&payload)?)
        .bind(at)
        .execute(&mut **tx)
        .await?
        .last_insert_rowid();
        stored.push(Stored {
            id,
            batch: payload.batch.unwrap_or(id),
            team_id,
            visible_to,
            by: payload.by,
            note: payload.note,
            at,
            entry: payload.entry,
        });
    }
    Ok(stored)
}

/// Hands committed entries to the open event streams.
pub fn publish(state: &AppState, stored: &[Stored]) {
    for entry in stored {
        // No receivers is fine.
        let _ = state.events.send(entry.clone());
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/{id}/events", get(events))
}

#[derive(Deserialize)]
struct After {
    after: Option<i64>,
}

fn to_sse(stored: &Stored) -> Result<SseEvent, Infallible> {
    Ok(SseEvent::default()
        .id(stored.id.to_string())
        .json_data(stored)
        .unwrap_or_else(|_| SseEvent::default().comment("unserializable entry")))
}

async fn events(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Query(query): Query<After>,
    headers: HeaderMap,
) -> ApiResult<Sse<impl Stream<Item = Result<SseEvent, Infallible>>>> {
    let after = headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .or(query.after)
        .unwrap_or(0);

    // Subscribe first, then read the log: nothing falls between the two.
    let receiver = state.events.subscribe();
    let rows = sqlx::query(
        "SELECT id, team_id, visible_to, payload, created_at FROM events WHERE team_id = ? AND id > ? ORDER BY id",
    )
    .bind(id)
    .bind(after)
    .fetch_all(&state.pool)
    .await?;
    let mut replay = Vec::with_capacity(rows.len());
    for row in rows {
        let payload: Payload = serde_json::from_str(row.get("payload"))?;
        let id: i64 = row.get("id");
        replay.push(Stored {
            id,
            batch: payload.batch.unwrap_or(id),
            team_id: row.get("team_id"),
            visible_to: row.get("visible_to"),
            by: payload.by,
            note: payload.note,
            at: row.get("created_at"),
            entry: payload.entry,
        });
    }
    // Access is checked after reading the log: a removal committed before
    // this point refuses the stream, a later one arrives live and ends it.
    let team = game::load_team(&state.pool, id).await?;
    let is_gm = game::access(&state.pool, &team, &user).await?;
    let last = replay.last().map_or(after, |s| s.id);
    let replay = stream::iter(
        replay
            .into_iter()
            .filter(move |s| visible(s, is_gm))
            .map(|s| to_sse(&s)),
    );
    // A stream that falls behind ends; the browser reconnects and catches
    // up from the log. After a character leaves the stream ends as well, so
    // the reconnect checks again who may still read along.
    let live = BroadcastStream::new(receiver)
        .take_while(|item| std::future::ready(item.is_ok()))
        .filter_map(move |item| {
            let ready = item
                .ok()
                .filter(|s| s.team_id == id && s.id > last && visible(s, is_gm));
            std::future::ready(ready)
        })
        .flat_map(|stored| {
            let leaves = matches!(
                stored.entry,
                Entry::CharacterRemoved { .. } | Entry::CharacterLeft { .. }
            );
            stream::iter(std::iter::once(Some(stored)).chain(leaves.then_some(None)))
        })
        .take_while(|item| std::future::ready(item.is_some()))
        .filter_map(|item| std::future::ready(item.map(|s| to_sse(&s))));
    Ok(Sse::new(replay.chain(live)).keep_alive(KeepAlive::default()))
}
