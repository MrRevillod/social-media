use std::sync::Arc;

use database::{models::User, DbConnectionRef};

pub struct UserRepository {
    pool: Arc<DbConnectionRef>,
}

impl UserRepository {
    pub fn new(pool: Arc<DbConnectionRef>) -> Self {
        UserRepository { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, ()> {
        Err(())
    }

    pub async fn find_one_by_id(&self, id: String) -> Result<Option<User>, ()> {
        Err(())
    }
}
