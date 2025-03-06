use common::{
    constants::{POSTGRES_DATABASE_URL, POSTGRES_TEST_DATABASE_URL},
    database::{PgPoolRef, PostgresClient},
    repositories::user::UserRepository,
};

use std::{env, ops::Index};

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: cargo run -- [mode (test | dev)]");
        return;
    }

    let url = match args.index(1).as_str() {
        "dev" => POSTGRES_DATABASE_URL.clone(),
        "test" => POSTGRES_TEST_DATABASE_URL.clone(),
        _ => {
            println!("Invalid mode");
            return;
        }
    };

    let pg_pool = PostgresClient::new(&url).await;

    println!("Seeding database...");

    match args.index(1).as_str() {
        "dev" => dev_seed(pg_pool).await,
        "test" => test_seed(pg_pool).await,
        _ => {
            println!("Invalid mode");
        }
    }
}

async fn dev_seed(pg_pool: PgPoolRef) {
    sqlx::query("DELETE FROM users")
        .execute(pg_pool.as_ref())
        .await
        .unwrap();

    let _ = UserRepository::create(
        &pg_pool,
        String::from("test_username"),
        String::from("test@mail.com"),
        String::from("!T3st_P4ssw0rd"),
    )
    .await;

    let _ = UserRepository::create(
        &pg_pool,
        String::from("test_username2"),
        String::from("lr@dev.com"),
        String::from("!abc1234ABC"),
    )
    .await;
}

async fn test_seed(pg_pool: PgPoolRef) {
    let username = String::from("test_username");
    let email = String::from("test@mail.com");
    let password = String::from("!T3st_P4ssw0rd");

    sqlx::query("DELETE FROM users")
        .execute(pg_pool.as_ref())
        .await
        .unwrap();

    let _ = UserRepository::create(&pg_pool, username, email, password).await;
}
