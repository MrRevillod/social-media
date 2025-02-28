use crate::{database::PgPoolRef, models::User};

use sqlx::{Error as SqlxError, QueryBuilder};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_all(pool: &PgPoolRef) -> Result<Vec<User>, SqlxError> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool.as_ref())
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(pool: &PgPoolRef, id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool.as_ref())
            .await?;

        Ok(user)
    }

    #[allow(unused_assignments)]
    pub async fn find_one(
        pool: &PgPoolRef,
        id: Option<&String>,
        email: Option<&String>,
    ) -> Result<Option<User>, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("SELECT * FROM users ");

        if id.is_some() || email.is_some() {
            query_builder.push("WHERE ");

            let mut conditions_added = 0;

            if let Some(id) = id {
                query_builder.push("id = ");
                query_builder.push_bind(id);
                conditions_added += 1;
            }

            if let Some(email) = email {
                if conditions_added > 0 {
                    query_builder.push(" AND ");
                }
                query_builder.push("email = ");
                query_builder.push_bind(email);
                conditions_added += 1;
            }
        }

        let query = query_builder.build_query_as::<User>();
        let user = query.fetch_optional(pool.as_ref()).await?;

        Ok(user)
    }

    pub async fn create(
        pool: &PgPoolRef,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(username)
        .bind(email)
        .bind(password)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(user)
    }
}
