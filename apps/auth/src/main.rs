use axum::routing::Router;
use common::{database::postgres::get_db_connection, services::state::AppState};
use tower_cookies::CookieManagerLayer;

mod session;
mod users;

#[tokio::main]
async fn main() {
    common::check_env_vars();

    let database = get_db_connection().await;
    let app_state = AppState::new(database.clone());

    let users_router = users::router::users_router(app_state.clone());
    let session_router = session::router::session_router(app_state.clone());

    let cookie_layer = CookieManagerLayer::new();

    let app = Router::new()
        .merge(session_router)
        .merge(users_router)
        .with_state(app_state)
        .layer(cookie_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
