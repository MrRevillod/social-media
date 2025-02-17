use axum::routing::Router;
use common::{database::postgre::init_prisma_client, services::state::AppState};

mod session;

#[tokio::main]
async fn main() {
    let prisma_client = init_prisma_client().await;
    let app_state = AppState::new(prisma_client);

    let session_router = session::router::session_router(app_state.clone());

    let app = Router::new()
        .nest("/session", session_router)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
