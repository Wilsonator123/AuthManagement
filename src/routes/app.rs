use axum::{Router, routing::post};

pub fn routes() -> Router {
    let router = Router::new().route("/app", post(app_post));
    return router;
}

async fn app_post() -> &'static str {
    return "Create new application";
}
