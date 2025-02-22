use axum::{
    middleware::from_fn_with_state,
    routing::{post, Router},
};
use common::services::{auth::AuthService, state::AppStateRef};

use super::controllers::*;

pub fn session_router(state: AppStateRef) -> Router<AppStateRef> {
    Router::new()
        .route("/login", post(login))
        .route(
            "/logout",
            post(logout).layer(from_fn_with_state(state.clone(), AuthService::authenticate)),
        )
        .with_state(state)
}
