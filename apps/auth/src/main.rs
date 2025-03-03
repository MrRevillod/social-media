use axum::{http::HeaderValue, Router};

use common::{
    check_env_vars,
    constants::BASE_SERVER_URL,
    database::PostgresClient,
    services::{logger::HttpLogger, state::AppState},
    utils::http::{ALLOWED_HTTP_HEADERS, ALLOWED_HTTP_METHODS},
};

use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

mod session;
mod users;

#[tokio::main]
async fn main() {
    check_env_vars();

    let database = PostgresClient::new().await;
    let app_state = AppState::new(database.clone());

    let users_router = users::router(app_state.clone());
    let session_router = session::router(app_state.clone());

    let http_logger = HttpLogger::new();
    let cookie_layer = CookieManagerLayer::new();

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(ALLOWED_HTTP_METHODS.to_owned())
        .allow_headers(ALLOWED_HTTP_HEADERS.to_owned())
        .allow_origin(BASE_SERVER_URL.parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .merge(session_router)
        .merge(users_router)
        .layer(cors)
        .layer(cookie_layer)
        .layer(http_logger.layer);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Auth service listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
