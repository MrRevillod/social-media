pub mod controllers;
pub mod schemas;

use axum::{routing::post, Router};
use common::services::state::AppState;

use self::controllers::*;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(create))
        .with_state(state)
}
