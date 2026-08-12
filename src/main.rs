use anyhow::Result;
use axum::{Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/", get(root));

    let listener = TcpListener::bind("0.0.0.0:6969").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn root() -> &'static str {
    "Hello, World!"
}
