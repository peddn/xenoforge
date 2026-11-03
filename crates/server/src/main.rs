use std::path::PathBuf;

use tracing_subscriber::EnvFilter;

/// Address the server listens on, unless `XENOFORGE_ADDR` is set.
const DEFAULT_ADDR: &str = "127.0.0.1:3000";
/// Built frontend, unless `XENOFORGE_STATIC` is set.
const DEFAULT_STATIC: &str = "frontend/dist";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let addr = std::env::var("XENOFORGE_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_owned());
    let static_dir = PathBuf::from(
        std::env::var("XENOFORGE_STATIC").unwrap_or_else(|_| DEFAULT_STATIC.to_owned()),
    );

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Xenoforge listening on http://{addr}");
    axum::serve(listener, xenoforge_server::app(&static_dir)).await
}
