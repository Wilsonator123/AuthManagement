use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres};
#[derive(Serialize, Deserialize)]
struct App {
    id: Uuid,
    name: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CreateApp {
    name: String,
    description: Option<String>,
}

pub fn routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/app", post(create_app))
        .route("/app/{id}", get(get_app))
        .layer(Extension(pool))
}

async fn get_app(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<App>, StatusCode> {
    let row = sqlx::query_as!(App, "select * from app where id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("User with ID: {} not found: {}", id, e);
            StatusCode::BAD_REQUEST
        })?;

    Ok(Json(row))
}

async fn create_app(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_app): Json<CreateApp>,
) -> Result<Json<Uuid>, StatusCode> {
    let id = Uuid::new_v4();
    let row = sqlx::query!(
        "INSERT INTO app(id, name, description) values ($1, $2, $3) RETURNING id",
        id,
        new_app.name,
        new_app.description
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create app: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(row.id))
}
