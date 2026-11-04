//! Web server for Xenoforge (plan section 11; server notes in
//! `docs/server.md`).
//!
//! Commands come in over HTTP, events go out as Server-Sent Events. The
//! rules live in the engine; the server stores state in SQLite, checks
//! rights and filters what each person may see.

pub mod auth;
pub mod cast;
pub mod characters;
pub mod error;
pub mod game;
pub mod log;
pub mod teams;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::{Json, Router, routing::get};
use serde::Serialize;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Sqlite, SqlitePool, Transaction};
use tokio::sync::{Mutex, broadcast};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use xenoforge_engine::Engine;
use xenoforge_engine::data::{Data, validate};
use xenoforge_engine::dice::Dice;

use crate::error::ApiError;

/// Where things are.
#[derive(Debug, Clone)]
pub struct Config {
    /// SQLite file, e.g. `xenoforge.db`.
    pub database: PathBuf,
    /// Built frontend.
    pub static_dir: PathBuf,
    /// Rule data (`data/`).
    pub data_dir: PathBuf,
    /// Mark cookies `Secure` (behind HTTPS).
    pub secure_cookies: bool,
}

/// Shared by all requests.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub engine: Arc<Engine>,
    /// New log entries for the event streams.
    pub events: broadcast::Sender<log::Stored>,
    /// Commands run one after another, so no update gets lost.
    pub lock: Arc<Mutex<()>>,
    /// The operating system's random source; tests put fixed dice here.
    pub dice: Arc<std::sync::Mutex<Box<dyn Dice + Send>>>,
    pub secure_cookies: bool,
}

/// Answer of `GET /api/health`.
#[derive(Debug, Serialize)]
pub struct Health {
    pub status: &'static str,
    pub version: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Opens the database, runs migrations, loads and checks the rule data and
/// builds the router.
pub async fn app(config: &Config) -> Result<Router, Box<dyn std::error::Error>> {
    let state = state(config).await?;
    Ok(router(state, &config.static_dir, &config.data_dir))
}

/// The shared state: database (migrated), rules (checked), dice.
pub async fn state(config: &Config) -> Result<AppState, Box<dyn std::error::Error>> {
    let data = Data::load(&config.data_dir)?;
    let problems = validate(&data);
    if !problems.is_empty() {
        let list: Vec<String> = problems.iter().map(ToString::to_string).collect();
        return Err(format!("rule data has problems:\n{}", list.join("\n")).into());
    }
    let options = SqliteConnectOptions::new()
        .filename(&config.database)
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let (events, _) = broadcast::channel(1024);
    let state = AppState {
        pool,
        engine: Arc::new(Engine::new(data.rules)),
        events,
        lock: Arc::new(Mutex::new(())),
        dice: Arc::new(std::sync::Mutex::new(Box::new(OsDice))),
        secure_cookies: config.secure_cookies,
    };
    Ok(state)
}

/// Everything below `/api`, the rule data under `/api/data` and the
/// frontend for all other paths (unknown ones get `index.html`).
pub fn router(state: AppState, static_dir: &Path, data_dir: &Path) -> Router {
    let frontend =
        ServeDir::new(static_dir).fallback(ServeFile::new(static_dir.join("index.html")));
    let api = Router::new()
        .route("/health", get(health))
        .nest("/auth", auth::routes())
        .nest("/characters", characters::routes())
        .nest(
            "/teams",
            teams::routes()
                .merge(game::routes())
                .merge(log::routes())
                .merge(cast::routes()),
        )
        .nest("/join", teams::join_routes())
        .nest_service("/data", ServeDir::new(data_dir))
        .fallback(|| async { ApiError::new(StatusCode::NOT_FOUND, "not_found", "no such API") })
        .with_state(state);
    Router::new()
        .nest("/api", api)
        .fallback_service(frontend)
        .layer(middleware::from_fn(cache_control))
        .layer(TraceLayer::new_for_http())
}

/// How long browsers keep files. Built files under `/assets/` carry a hash
/// in their name and never change. The page itself and the rule data are
/// checked on every load, so a new build shows up after a normal reload.
/// API answers stay as they are.
async fn cache_control(request: Request, next: Next) -> Response {
    let path = request.uri().path();
    let value = if path.starts_with("/assets/") {
        Some("public, max-age=31536000, immutable")
    } else if path.starts_with("/api/") && !path.starts_with("/api/data/") {
        None
    } else {
        Some("no-cache")
    };
    let mut response = next.run(request).await;
    if let Some(value) = value {
        response
            .headers_mut()
            .insert(header::CACHE_CONTROL, HeaderValue::from_static(value));
    }
    response
}

/// A transaction that writes: it takes SQLite's write lock at once, so a
/// write elsewhere cannot make it fail halfway (`SQLITE_BUSY_SNAPSHOT`).
pub async fn begin_write(pool: &SqlitePool) -> Result<Transaction<'static, Sqlite>, ApiError> {
    Ok(pool.begin_with("BEGIN IMMEDIATE").await?)
}

/// Seconds since 1970.
pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

/// 32 random bytes from the operating system, as hex.
pub fn random_token() -> Result<String, ApiError> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|e| ApiError::internal(format!("random: {e}")))?;
    Ok(hex::encode(bytes))
}

/// The server's dice: the operating system's random source (plan 11.3).
pub struct OsDice;

impl Dice for OsDice {
    fn d6(&mut self) -> u8 {
        // Reject the top values so every face is equally likely.
        const LIMIT: u32 = u32::MAX - u32::MAX % 6;
        loop {
            let value = getrandom::u32().expect("the operating system's random source works");
            if value < LIMIT {
                return u8::try_from(value % 6).unwrap_or(0) + 1;
            }
        }
    }
}
