use common::create_app;
use tokio::net::TcpListener;

mod session;
mod users;

#[tokio::main]
async fn main() {
    let app = create_app(vec![users::router, session::router]).await;
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Auth service listening on port 8000");

    axum::serve(listener, app).await.unwrap();
}
