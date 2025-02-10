use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    let app = Router::new();
    let db = database::init().await;

    axum::serve(listener, app).await.unwrap();
}
