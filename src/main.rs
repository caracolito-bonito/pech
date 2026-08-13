use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
struct Pie {
    id: Uuid,
    name: String,
    code: String,
}
#[derive(Deserialize)]
struct CreatePie {
    name: String,
    code: String,
}

type Storage = Arc<RwLock<HashMap<Uuid, Pie>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let storage = Storage::default();

    let app = Router::new()
        .route("/health", get(health))
        .route("/", get(root))
        .route("/pies", post(create_pie))
        .route("/pies/{id}", get(get_pie).delete(delete_pie))
        .with_state(storage);

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

async fn create_pie(
    State(storage): State<Storage>,
    Json(payload): Json<CreatePie>,
) -> impl IntoResponse {
    let pie = Pie {
        id: Uuid::new_v4(),
        name: payload.name,
        code: payload.code,
    };

    storage.write().unwrap().insert(pie.id, pie.clone());

    (StatusCode::CREATED, Json(pie))
}

async fn get_pie(
    State(storage): State<Storage>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let pie = storage
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(pie))
}

async fn delete_pie(State(storage): State<Storage>, Path(id): Path<Uuid>) -> impl IntoResponse {
    if storage.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
