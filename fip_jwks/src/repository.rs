use crate::{config::Config, model::Model};
use fip_common::common_error::CommonError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// TODO: documentation
#[derive(Debug)]
pub struct Repository {
    config: Config,
    pool: SqlitePool,
}

impl Repository {
    /// TODO: documentation
    pub async fn new(config: Config) -> Self {
        let pool = SqlitePoolOptions::new()
            .connect(&config.clone().database_url())
            .await
            .unwrap();
        Self { config, pool }
    }
}

impl Repository {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete(&self, id: &str) -> Result<u64, CommonError> {
        let done = sqlx::query("DELETE FROM jwks WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find(&self) -> Result<Vec<Model>, CommonError> {
        sqlx::query_as("SELECT id, private_key, public_key FROM jwks")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
            })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, id: &str) -> Result<Model, CommonError> {
        sqlx::query_as("SELECT id, private_key, public_key FROM jwks WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
            })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_random(&self) -> Result<Model, CommonError> {
        sqlx::query_as("SELECT id, private_key, public_key FROM jwks ORDER BY Random() LIMIT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
            })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn save(&self, model: &Model) -> Result<u64, CommonError> {
        let done =
            sqlx::query("INSERT INTO jwks (id, private_key, public_key FROM) VALUES (?, ?, ?);")
                .bind(model.id.clone())
                .bind(model.private_key.clone())
                .bind(model.public_key.clone())
                .execute(&self.pool)
                .await
                .map_err(|err| {
                    tracing::error!("{:?}", err);
                    err
                })?;
        Ok(done.rows_affected())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn update(&self, model: &Model) -> Result<u64, CommonError> {
        let done = sqlx::query("UPDATE jwks SET private_key = ?, public_key = ? WHERE id = ?")
            .bind(model.private_key.clone())
            .bind(model.public_key.clone())
            .bind(model.id.clone())
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }
}
