use axum::{Router, routing::method_routing::post};

mod create;
mod vote;

pub fn routes() -> Router {
    Router::new()
        .route("/vote/{*id}", post(vote::post))
        .route("/create", post(create::post))
}
