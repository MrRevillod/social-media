use crate::database::postgre::{
    self,
    prisma::user::{self, Data, SetParam},
};

use prisma_client_rust::QueryError;

#[warn(dead_code)]
pub struct UserRepository {
    client: postgre::DbConnectionRef,
}

impl UserRepository {
    pub fn new(client: postgre::DbConnectionRef) -> Self {
        UserRepository { client }
    }

    pub async fn find_all(&self) -> Result<Vec<Data>, QueryError> {
        Ok(self.client.user().find_many(vec![]).exec().await?)
    }

    pub async fn find_one_by_id(&self, id: &String) -> Result<Option<Data>, QueryError> {
        Ok(self
            .client
            .user()
            .find_unique(user::id::equals(id.clone()))
            .exec()
            .await?)
    }

    pub async fn create(
        &self,
        name: String,
        email: String,
        password: String,
        validated: Option<bool>,
    ) -> Result<Data, QueryError> {
        Ok(self
            .client
            .user()
            .create(name, email, password, validated.unwrap_or(false), vec![])
            .exec()
            .await?)
    }

    pub async fn update_one(&self, id: &String, fields: Vec<SetParam>) -> Result<Data, QueryError> {
        Ok(self
            .client
            .user()
            .update(user::id::equals(id.clone()), fields)
            .exec()
            .await?)
    }
}
