use axum::{routing::get, Router, response::Redirect};
use tracing::info;
use tracing_subscriber::EnvFilter;

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

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    info!("Listening on {}", address);

    let _ = axum::serve(listener, app).await;
}

async fn root() -> Redirect {
    Redirect::permanent("https://github.com/niko-at-chalupa/endstone-elytra-core")
}