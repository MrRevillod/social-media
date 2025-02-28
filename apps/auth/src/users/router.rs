use axum::routing::{post, Router};
use common::services::state::AppStateRef;

use super::controllers::*;

pub fn users_router(state: AppStateRef) -> Router<AppStateRef> {
    Router::new()
        .route("/register", post(create))
        .with_state(state)
}
