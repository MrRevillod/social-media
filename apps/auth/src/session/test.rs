use std::sync::Arc;

use axum::extract::Request;
use axum::response::Response;
use axum::{body::Body, http::HeaderMap};
use common::services::state::AppState;

use common::check_env_vars;
use common::database::PostgresClient;

use axum::{http::StatusCode, Router};
use serde_json::{json, Value};

use tower::ServiceExt;

use super::router;

#[cfg(test)]
async fn app() -> Router {
    check_env_vars();

    let database = PostgresClient::new().await;
    let app_state = AppState::new(Arc::clone(&database));

    router(app_state.clone())
}

#[cfg(test)]
async fn login_controller(data: Value) -> Response {
    let app = app().await;

    let request = Request::builder()
        .uri("/login")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(data.to_string()))
        .unwrap();

    app.oneshot(request).await.unwrap()
}

async fn validate_auth_controller(headers: HeaderMap) -> Response {
    let app = app().await;

    let mut request = Request::builder()
        .uri("/validate-session")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    headers.iter().for_each(|(key, value)| {
        request.headers_mut().insert(key, value.clone());
    });

    app.oneshot(request).await.unwrap()
}

// login_controller tests (OK, BAD_REQUEST, UNAUTHORIZED)

#[tokio::test]
async fn should_login_200() {
    let data = json!({
        "email": "test@mail.com ",
        "password": "!T3st_P4ssw0rd"
    });

    let response = login_controller(data).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_not_login_400() {
    let data = json!({
        "email": "",
        "password": ""
    });

    let response = login_controller(data).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_not_login_401() {
    let data = json!({
        "email": "wrong_email@domain.com",
        "password": "wrong_password"
    });

    let response = login_controller(data).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// validate_auth_controller tests (OK, UNAUTHORIZED)

#[tokio::test]
async fn should_validate_auth_200() {
    let data = json!({
        "email": "test@mail.com ",
        "password": "!T3st_P4ssw0rd"
    });

    let login_response = login_controller(data).await;
    let headers = login_response.headers().clone();
    let response = validate_auth_controller(headers).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_not_validate_auth_401() {
    let headers = HeaderMap::new();
    let response = validate_auth_controller(headers).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
