use std::path::PathBuf;
use std::process::ExitCode;

use tracing_subscriber::EnvFilter;

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

const USAGE: &str = "usage: xenoforge-server [--seed]\n  --seed  fill an empty database with sample data, then start";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut seed = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--seed" => seed = true,
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            other => return Err(format!("unknown argument \"{other}\"\n{USAGE}").into()),
        }
    }

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
    let state = xenoforge_server::state(&config).await?;
    let app = xenoforge_server::router(state.clone(), &config.static_dir, &config.data_dir);
    if seed {
        for line in xenoforge_server::seed::seed(app.clone(), &state.pool).await? {
            tracing::info!("sample data: {line}");
        }
    }
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Xenoforge listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
