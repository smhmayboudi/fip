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
        let done = sqlx::query("DELETE FROM user WHERE id = ?")
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
    pub async fn delete_by_cellphone(&self, cellphone: &str) -> Result<u64, Error> {
        let done = sqlx::query("DELETE FROM user WHERE cellphone = ?")
            .bind(cellphone)
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
    pub async fn delete_by_username(&self, username: &str) -> Result<u64, Error> {
        let done = sqlx::query("DELETE FROM user WHERE username = ?")
            .bind(username)
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
        sqlx::query_as(
            "SELECT avatar, biography, birth_date, cellphone, email, first_name, gender, id, language_code, last_name, password, registered_at, username FROM user",
        )
        .fetch_all(&self.pool)
        .await.map_err(|err| {
            tracing::error!("{:?}", err);
            err.into()
        })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, id: &str) -> Result<Model, Error> {
        sqlx::query_as(
            "SELECT avatar, biography, birth_date, cellphone, email, first_name, gender, id, language_code, last_name, password, registered_at, username FROM user WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await.map_err(|err| {
            tracing::error!("{:?}", err);
            err.into()
        })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_by_cellphone(&self, cellphone: &str) -> Result<Model, Error> {
        sqlx::query_as(
            "SELECT avatar, biography, birth_date, cellphone, email, first_name, gender, id, language_code, last_name, password, registered_at, username FROM user WHERE cellphone = ?",
        )
        .bind(cellphone)
        .fetch_one(&self.pool)
        .await.map_err(|err| {
            tracing::error!("{:?}", err);
            err.into()
        })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_by_username(&self, username: &str) -> Result<Model, Error> {
        sqlx::query_as(
            "SELECT avatar, biography, birth_date, cellphone, email, first_name, gender, id, language_code, last_name, password, registered_at, username FROM user WHERE username = ?",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await.map_err(|err| {
            tracing::error!("{:?}", err);
            err.into()
        })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn save(&self, model: &Model) -> Result<u64, Error> {
        let done = sqlx::query(
            "INSERT INTO user (avatar, biography, birth_date, cellphone, email, first_name, gender, id, language_code, last_name, password, registered_at, username, ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
        )
        .bind(model.avatar.clone())
        .bind(model.biography.clone())
        .bind(model.birth_date)
        .bind(model.cellphone.clone())
        .bind(model.email.clone())
        .bind(model.first_name.clone())
        .bind(model.gender.clone())
        .bind(model.id.clone())
        .bind(model.language_code.clone())
        .bind(model.last_name.clone())
        .bind(model.password.clone())
        .bind(model.registered_at)
        .bind(model.username.clone())
        .execute(&self.pool)
        .await.map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn update(&self, model: &Model) -> Result<u64, Error> {
        let done = sqlx::query(
            "UPDATE user SET avatar = ?, biography = ?, birth_date = ?, cellphone = ?, email = ?, first_name = ?, gender = ?, language_code = ?, last_name = ?, password = ?, registered_at = ?, username = ? WHERE id = ?",
        )
        .bind(model.avatar.clone())
        .bind(model.biography.clone())
        .bind(model.birth_date)
        .bind(model.cellphone.clone())
        .bind(model.email.clone())
        .bind(model.first_name.clone())
        .bind(model.gender.clone())
        .bind(model.language_code.clone())
        .bind(model.last_name.clone())
        .bind(model.password.clone())
        .bind(model.registered_at)
        .bind(model.username.clone())
        .bind(model.id.clone())
        .execute(&self.pool)
        .await.map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }
}
