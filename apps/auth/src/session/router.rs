use axum::routing::{post, Router};

use common::services::state::AppStateRef;

use super::controllers::login;

pub fn session_router(state: AppStateRef) -> Router<AppStateRef> {
    Router::new().route("/login", post(login)).with_state(state)
}
