//! Requests to the API inside the process, without a network: for the
//! sample data (`--seed`) and the tests. A client keeps the session cookie
//! like a browser, so every request goes through the same handlers and
//! checks as in play.

use std::fmt;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use serde_json::Value;
use tower::ServiceExt;

/// A request that did not succeed.
#[derive(Debug)]
pub struct Failed {
    pub method: String,
    pub path: String,
    pub status: StatusCode,
    pub body: Value,
}

impl fmt::Display for Failed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {} {}",
            self.method, self.path, self.status, self.body
        )
    }
}

impl std::error::Error for Failed {}

pub struct Client {
    app: Router,
    cookie: Option<String>,
}

impl Client {
    pub fn new(app: Router) -> Self {
        Self { app, cookie: None }
    }

    /// The session cookie, once logged in (`name=value`).
    pub fn cookie(&self) -> Option<&str> {
        self.cookie.as_deref()
    }

    /// Status and JSON answer (`null` if there is none).
    pub async fn call(
        &mut self,
        method: &str,
        path: &str,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        let mut request = Request::builder().method(method).uri(path);
        if let Some(cookie) = &self.cookie {
            request = request.header(header::COOKIE, cookie);
        }
        let request = match body {
            Some(body) => request
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string())),
            None => request.body(Body::empty()),
        };
        let Ok(request) = request else {
            return (StatusCode::BAD_REQUEST, Value::Null);
        };
        let response = match self.app.clone().oneshot(request).await {
            Ok(response) => response,
            Err(never) => match never {},
        };
        if let Some(set) = response.headers().get(header::SET_COOKIE) {
            if let Some(pair) = set.to_str().ok().and_then(|s| s.split(';').next()) {
                self.cookie = Some(pair.to_owned());
            }
        }
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, value)
    }

    /// The answer of a successful request.
    pub async fn ok(
        &mut self,
        method: &str,
        path: &str,
        body: Option<Value>,
    ) -> Result<Value, Failed> {
        let (status, value) = self.call(method, path, body).await;
        if status.is_success() {
            Ok(value)
        } else {
            Err(Failed {
                method: method.to_owned(),
                path: path.to_owned(),
                status,
                body: value,
            })
        }
    }
}
