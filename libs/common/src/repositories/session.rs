use prisma_client_rust::QueryError;

use super::SessionRepository;
use crate::{
    database::postgre::DbConnectionRef,
    prisma::{session, user},
};

impl SessionRepository {
    pub async fn find_all(client: DbConnectionRef) -> Result<Vec<session::Data>, QueryError> {
        Ok(client.session().find_many(vec![]).exec().await?)
    }

    pub async fn create(
        client: DbConnectionRef,
        token: String,
        user_id: String,
    ) -> Result<session::Data, QueryError> {
        Ok(client
            .session()
            .create(token, user::UniqueWhereParam::IdEquals(user_id), vec![])
            .exec()
            .await?)
    }

    pub async fn delete_one(
        client: DbConnectionRef,
        id: &String,
    ) -> Result<session::Data, QueryError> {
        Ok(client
            .session()
            .delete(session::id::equals(id.clone()))
            .exec()
            .await?)
    }
}
