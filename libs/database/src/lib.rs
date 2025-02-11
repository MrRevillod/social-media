pub mod models;
pub mod schema;

use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::{env, sync::Arc};

pub type DbConnectionRef = Arc<AsyncPgConnection>;

pub async fn init() -> AsyncPgConnection {
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    AsyncPgConnection::establish(&db_uri)
        .await
        .unwrap_or_else(|error| panic!("Error while connecting to pg database: {}", error))
}
