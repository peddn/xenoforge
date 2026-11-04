use std::path::PathBuf;

use tracing_subscriber::EnvFilter;

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let addr = env_or("XENOFORGE_ADDR", "127.0.0.1:3000");
    let config = xenoforge_server::Config {
        database: PathBuf::from(env_or("XENOFORGE_DB", "xenoforge.db")),
        static_dir: PathBuf::from(env_or("XENOFORGE_STATIC", "frontend/dist")),
        data_dir: PathBuf::from(env_or("XENOFORGE_DATA", "data")),
        secure_cookies: env_or("XENOFORGE_SECURE_COOKIES", "false") == "true",
    };
    let app = xenoforge_server::app(&config).await?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Xenoforge listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
