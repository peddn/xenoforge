//! Web server for Xenoforge.
//!
//! Phase 0 only offers a health check under `/api/health` and serves the
//! built frontend. Rules, accounts and the database follow in later phases.

use std::path::Path;

use axum::http::StatusCode;
use axum::{Json, Router, routing::get};
use serde::Serialize;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

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

/// Builds the router. Everything outside `/api` is served from
/// `static_dir`; unknown paths get `index.html`, so the Vue router can
/// handle them. Unknown paths below `/api` get 404.
pub fn app(static_dir: &Path) -> Router {
    let index = static_dir.join("index.html");
    let frontend = ServeDir::new(static_dir).fallback(ServeFile::new(index));

    let api = Router::new()
        .route("/health", get(health))
        .fallback(|| async { StatusCode::NOT_FOUND });

    Router::new()
        .nest("/api", api)
        .fallback_service(frontend)
        .layer(TraceLayer::new_for_http())
}
