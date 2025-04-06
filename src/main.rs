use axum::{Extension, Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing::{Level, info};
use tracing_subscriber;

mod config;
mod routes;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let pool = config::database::open_connection().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .merge(routes::app::routes(pool.clone()))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// handler for GET /
async fn root() -> &'static str {
    "Hello, world!"
}
