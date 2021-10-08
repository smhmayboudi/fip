use crate::{config::Config, model::Model};
use fip_common::error::Error;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// TODO: documentation
#[derive(Debug)]
pub struct Repository {
    config: Config,
    pool: SqlitePool,
}

/// TODO: documentation
impl Repository {
    /// TODO: documentation
    ///
    /// # Panics
    /// TODO: documentation panics
    pub async fn new(config: Config) -> Self {
        let pool = SqlitePoolOptions::new()
            .connect(&config.clone().database_url())
            .await
            .unwrap_or_else(|err| {
                panic!("{:?}", err);
            });
        Self { config, pool }
    }
}

/// TODO: documentation
impl Repository {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete(&self, id: &str) -> Result<u64, Error> {
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
    pub async fn find(&self) -> Result<Vec<Model>, Error> {
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
    pub async fn find_one(&self, id: &str) -> Result<Model, Error> {
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
    pub async fn find_one_random(&self) -> Result<Model, Error> {
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
    pub async fn save(&self, model: &Model) -> Result<u64, Error> {
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
    pub async fn update(&self, model: &Model) -> Result<u64, Error> {
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
