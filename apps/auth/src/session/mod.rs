pub mod controllers;
pub mod schemas;

#[cfg(test)]
pub mod test;

use axum::{middleware::from_fn_with_state as mw, routing::post, Router};
use common::services::{auth::AuthService, state::AppState};

use self::controllers::*;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route(
            "/logout",
            post(logout).layer(mw(state.clone(), AuthService::authenticate)),
        )
        .route("/refresh", post(refresh))
        .route(
            "/validate-session",
            post(validate_session).layer(mw(state.clone(), AuthService::authenticate)),
        )
        .with_state(state)
}
