use axum::{routing::method_routing::post, Router};

mod create;
mod vote;

pub fn routes() -> Router {
    Router::new()
        .route("/vote/{*id}", post(vote::post))
        .route("/create", post(vote::post))
}
