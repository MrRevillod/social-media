use prisma_client_rust::QueryError;

use super::UserRepository;
use crate::database::postgre::DbConnectionRef;
use crate::prisma::user::{self, Data, SetParam, UniqueWhereParam};

impl UserRepository {
    pub async fn find_all(client: DbConnectionRef) -> Result<Vec<Data>, QueryError> {
        Ok(client.user().find_many(vec![]).exec().await?)
    }

    pub async fn find_one(
        client: &DbConnectionRef,
        filter: UniqueWhereParam,
    ) -> Result<Option<Data>, QueryError> {
        Ok(client.user().find_unique(filter).exec().await?)
    }

    pub async fn create(
        client: DbConnectionRef,
        name: String,
        email: String,
        password: String,
        validated: Option<bool>,
    ) -> Result<Data, QueryError> {
        Ok(client
            .user()
            .create(name, email, password, validated.unwrap_or(false), vec![])
            .exec()
            .await?)
    }

    pub async fn update_one(
        client: DbConnectionRef,
        id: &String,
        fields: Vec<SetParam>,
    ) -> Result<Data, QueryError> {
        Ok(client
            .user()
            .update(user::id::equals(id.clone()), fields)
            .exec()
            .await?)
    }
}
