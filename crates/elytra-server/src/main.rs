use axum::{routing::get, Router, response::Redirect};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod database;

#[cfg(feature = "endgit")]
mod endgit;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        info!("This server is running {} {}", name, version);
    }


    let app = Router::new()
        .route("/", get(root));

    let db = database::Database::connect("sqlite://plugins.db").await.expect("Failed to create database");
    let mut state = AppState::new(db);

    #[cfg(feature = "endgit")]
    {
        info!("Endgit features are enabled, finding and loading Endgit plugins...");
        match state.endgit.fill().await {
            Ok(_) => (),
            Err(e) => {
                std::panic::panic_any(e);
            }
        }

        let plugins_hashmap = state.endgit.plugins();
        info!("Found & loaded {} Endgit plugins.", plugins_hashmap.len());
    }

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    info!("Listening on {}", address);

    let _ = axum::serve(listener, app).await;
}

struct AppState {
    db: database::Database,
    #[cfg(feature = "endgit")]
    endgit: endgit::Endgit,
}

impl AppState {
    pub fn new(db: database::Database) -> Self {
        #[cfg(feature = "endgit")]
        let endgit = match endgit::Endgit::new() {
            Ok(v) => v,
            Err(e) => {
                std::panic::panic_any(e);
            }
        };
        Self {
            db: db,
            #[cfg(feature = "endgit")]
            endgit: endgit,
        }
    }
}

async fn root() -> Redirect {
    Redirect::permanent("https://github.com/niko-at-chalupa/endstone-elytra-core")
}