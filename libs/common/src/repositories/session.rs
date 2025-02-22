use crate::{database::postgres::PgPoolRef, models::Session};
use chrono::{DateTime, Utc};
use sqlx::Error as SqlxError;
use uuid::Uuid;

pub struct SessionRepository;

impl SessionRepository {
    pub async fn find_all(pool: &PgPoolRef) -> Result<Vec<Session>, SqlxError> {
        let sessions = sqlx::query_as::<_, Session>("SELECT * FROM sessions")
            .fetch_all(pool.as_ref())
            .await?;

        Ok(sessions)
    }

    pub async fn create(
        pool: &PgPoolRef,
        token: String,
        user_id: Uuid,
        ip_address: Option<String>,
        user_agent: Option<String>,
        expires_at: u64,
    ) -> Result<Session, SqlxError> {
        let session = sqlx::query_as::<_, Session>(
            "INSERT INTO sessions (token, user_id, ip_address, user_agent, active, expires_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(token)
        .bind(user_id)
        .bind(ip_address)
        .bind(user_agent)
        .bind(true)
        .bind(DateTime::<Utc>::from_timestamp(expires_at as i64, 0))
        .fetch_one(pool.as_ref())
        .await?;

        Ok(session)
    }

    pub async fn delete_one(pool: &PgPoolRef, id: String) -> Result<(), SqlxError> {
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(id)
            .execute(pool.as_ref())
            .await?;

        Ok(())
    }

    pub async fn desactivate(pool: &PgPoolRef, id: String) -> Result<Session, SqlxError> {
        let session =
            sqlx::query_as::<_, Session>("UPDATE sessions SET active = false WHERE id = $1")
                .bind(id)
                .fetch_one(pool.as_ref())
                .await?;

        Ok(session)
    }

    pub async fn find_one(
        pool: &PgPoolRef,
        session_id: &String,
    ) -> Result<Option<Session>, SqlxError> {
        let session = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = $1")
            .bind(session_id)
            .fetch_optional(pool.as_ref())
            .await?;

        Ok(session)
    }
}
