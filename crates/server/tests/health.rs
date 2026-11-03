use std::path::Path;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn get(path: &str, static_dir: &Path) -> (StatusCode, Vec<u8>) {
    let response = xenoforge_server::app(static_dir)
        .oneshot(Request::get(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, body.to_vec())
}

/// A fake frontend build, one directory per test.
fn static_dir(test: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("xenoforge-{}-{test}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("index.html"),
        "<!doctype html><title>index</title>",
    )
    .unwrap();
    dir
}

#[tokio::test]
async fn health_reports_ok() {
    let (status, body) = get("/api/health", &static_dir("health")).await;
    assert_eq!(status, StatusCode::OK);
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["version"], env!("CARGO_PKG_VERSION"));
}

#[tokio::test]
async fn unknown_paths_get_the_frontend() {
    let (status, body) = get("/characters/42", &static_dir("spa")).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        String::from_utf8(body)
            .unwrap()
            .contains("<title>index</title>")
    );
}

#[tokio::test]
async fn unknown_api_paths_are_not_found() {
    let (status, _) = get("/api/nothing", &static_dir("api")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
