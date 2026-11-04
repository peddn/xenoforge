//! Accounts and login sessions (A-07). Passwords are stored with argon2;
//! the session cookie holds a random token whose SHA-256 is stored.

use argon2::Argon2;
use argon2::password_hash::phc::PasswordHash;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};
use axum::extract::{FromRequestParts, State};
use axum::http::header::{COOKIE, SET_COOKIE};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router, routing::get, routing::post};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::Row;

use crate::error::{ApiError, ApiResult};
use crate::{AppState, now, random_token};

pub const COOKIE_NAME: &str = "xf_session";
/// Sessions last 30 days.
const SESSION_SECONDS: i64 = 30 * 24 * 3600;

/// Player or game master, chosen at registration (A-07).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Player,
    Gm,
}

impl Role {
    fn as_str(self) -> &'static str {
        match self {
            Role::Player => "player",
            Role::Gm => "gm",
        }
    }

    fn parse(text: &str) -> Role {
        if text == "gm" { Role::Gm } else { Role::Player }
    }
}

/// The logged-in user, taken from the session cookie.
#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub role: Role,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

#[derive(Deserialize)]
struct Register {
    name: String,
    password: String,
    role: Role,
}

#[derive(Deserialize)]
struct Login {
    name: String,
    password: String,
}

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

fn session_cookie(state: &AppState, token: &str, max_age: i64) -> HeaderValue {
    let secure = if state.secure_cookies { "; Secure" } else { "" };
    let value =
        format!("{COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}");
    HeaderValue::from_str(&value).expect("cookie is ASCII")
}

async fn start_session(state: &AppState, user: User) -> ApiResult<Response> {
    let token = random_token()?;
    sqlx::query("INSERT INTO sessions (token_hash, user_id, created_at) VALUES (?, ?, ?)")
        .bind(hash_token(&token))
        .bind(user.id)
        .bind(now())
        .execute(&state.pool)
        .await?;
    let mut response = Json(user).into_response();
    response
        .headers_mut()
        .insert(SET_COOKIE, session_cookie(state, &token, SESSION_SECONDS));
    Ok(response)
}

async fn register(
    State(state): State<AppState>,
    Json(body): Json<Register>,
) -> ApiResult<Response> {
    let name = body.name.trim().to_owned();
    if !(2..=40).contains(&name.chars().count()) {
        return Err(ApiError::bad_request("the name needs 2 to 40 characters"));
    }
    if body.password.chars().count() < 8 {
        return Err(ApiError::bad_request(
            "the password needs at least 8 characters",
        ));
    }
    let hash = Argon2::default()
        .hash_password(body.password.as_bytes())
        .map_err(|e| ApiError::internal(format!("hashing: {e}")))?
        .to_string();
    let result = sqlx::query(
        "INSERT INTO users (name, password_hash, role, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&name)
    .bind(hash)
    .bind(body.role.as_str())
    .bind(now())
    .execute(&state.pool)
    .await;
    let id = match result {
        Ok(done) => done.last_insert_rowid(),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            return Err(ApiError::conflict("this name is taken"));
        }
        Err(e) => return Err(e.into()),
    };
    start_session(
        &state,
        User {
            id,
            name,
            role: body.role,
        },
    )
    .await
}

async fn login(State(state): State<AppState>, Json(body): Json<Login>) -> ApiResult<Response> {
    let row = sqlx::query("SELECT id, name, password_hash, role FROM users WHERE name = ?")
        .bind(body.name.trim())
        .fetch_optional(&state.pool)
        .await?;
    let wrong = || {
        ApiError::new(
            axum::http::StatusCode::UNAUTHORIZED,
            "login_failed",
            "wrong name or password",
        )
    };
    let Some(row) = row else { return Err(wrong()) };
    let stored: String = row.get("password_hash");
    let parsed =
        PasswordHash::new(&stored).map_err(|e| ApiError::internal(format!("stored hash: {e}")))?;
    if Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed)
        .is_err()
    {
        return Err(wrong());
    }
    let role: String = row.get("role");
    start_session(
        &state,
        User {
            id: row.get("id"),
            name: row.get("name"),
            role: Role::parse(&role),
        },
    )
    .await
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> ApiResult<Response> {
    if let Some(token) = cookie_token(&headers) {
        sqlx::query("DELETE FROM sessions WHERE token_hash = ?")
            .bind(hash_token(&token))
            .execute(&state.pool)
            .await?;
    }
    let mut response = Json(serde_json::json!({ "ok": true })).into_response();
    response
        .headers_mut()
        .insert(SET_COOKIE, session_cookie(&state, "", 0));
    Ok(response)
}

async fn me(user: User) -> Json<User> {
    Json(user)
}

fn cookie_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get_all(COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(name, _)| *name == COOKIE_NAME)
        .map(|(_, value)| value.to_owned())
        .filter(|v| !v.is_empty())
}

impl FromRequestParts<AppState> for User {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = cookie_token(&parts.headers).ok_or_else(ApiError::unauthorized)?;
        let row = sqlx::query(
            "SELECT users.id, users.name, users.role, sessions.created_at FROM sessions \
             JOIN users ON users.id = sessions.user_id WHERE sessions.token_hash = ?",
        )
        .bind(hash_token(&token))
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(ApiError::unauthorized)?;
        let created: i64 = row.get("created_at");
        if now() - created > SESSION_SECONDS {
            return Err(ApiError::unauthorized());
        }
        let role: String = row.get("role");
        Ok(User {
            id: row.get("id"),
            name: row.get("name"),
            role: Role::parse(&role),
        })
    }
}
