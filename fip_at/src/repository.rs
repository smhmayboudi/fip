use crate::{config::Config, model::Model};
use chrono::Utc;
use fip_common::common_error::CommonError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[derive(Debug)]
pub struct Repository {
    config: Config,
    pool: SqlitePool,
}

impl Repository {
    pub async fn new(config: Config) -> Self {
        let pool = SqlitePoolOptions::default()
            .connect(&config.clone().database_url())
            .await
            .unwrap();
        Self { config, pool }
    }
}

impl Repository {
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete(&self, claims_jti: &str) -> Result<u64, CommonError> {
        let done = sqlx::query("DELETE FROM at WHERE claims_jti = ?")
            .bind(claims_jti)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete_by_claims_sub(&self, claims_sub: &str) -> Result<u64, CommonError> {
        let done = sqlx::query("DELETE FROM at WHERE claims_sub = ?")
            .bind(claims_sub)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find(&self) -> Result<Vec<Model>, CommonError> {
        sqlx::query_as(
            "SELECT claims_aud, claims_exp, claims_iat, claims_iss, claims_jti, claims_nbf, claims_sub, header_typ, header_alg, header_cty, header_jku, header_kid, header_x5u, header_x5t, token_blocked, token_blocked_description FROM at",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("{:?}", err);
            err.into()
        })
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, claims_jti: &str) -> Result<Model, CommonError> {
        sqlx::query_as(
            "SELECT claims_aud, claims_exp, claims_iat, claims_iss, claims_jti, claims_nbf, claims_sub, header_typ, header_alg, header_cty, header_jku, header_kid, header_x5u, header_x5t, token_blocked, token_blocked_description FROM at WHERE claims_jti = ?",
        )
        .bind(claims_jti)
        .fetch_one(&self.pool)
        .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
        })
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_by_claims_sub(&self, claims_sub: &str) -> Result<Model, CommonError> {
        sqlx::query_as(
            "SELECT claims_aud, claims_exp, claims_iat, claims_iss, claims_jti, claims_nbf, claims_sub, header_typ, header_alg, header_cty, header_jku, header_kid, header_x5u, header_x5t, token_blocked, token_blocked_description FROM at WHERE claims_sub = ?",
        )
        .bind(claims_sub)
        .fetch_one(&self.pool)
        .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
        })
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn save(&self, model: &Model) -> Result<u64, CommonError> {
        let done = sqlx::query(
            "INSERT INTO at (claims_aud, claims_exp, claims_iat, claims_iss, claims_jti, claims_nbf, claims_sub, header_typ, header_alg, header_cty, header_jku, header_kid, header_x5u, header_x5t, token_blocked, token_blocked_description) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
        )
        .bind(model.claims_aud.clone())
        .bind(model.claims_exp)
        .bind(model.claims_iat)
        .bind(model.claims_iss.clone())
        .bind(model.claims_jti.clone())
        .bind(model.claims_nbf)
        .bind(model.claims_sub.clone())
        .bind(model.header_typ.clone())
        .bind(model.header_alg.clone())
        .bind(model.header_cty.clone())
        .bind(model.header_jku.clone())
        .bind(model.header_kid.clone())
        .bind(model.header_x5u.clone())
        .bind(model.header_x5t.clone())
        .bind(model.token_blocked)
        .bind(model.token_blocked_description.clone())
        .execute(&self.pool)
        .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn update(&self, model: &Model) -> Result<u64, CommonError> {
        let done = sqlx::query(
                "UPDATE at SET claims_aud = ?, claims_exp = ?, claims_iat = ?, claims_iss = ?, claims_nbf = ?, claims_sub = ?, header_typ = ?, header_alg = ?, header_cty = ?, header_jku = ?, header_kid = ?, header_x5u = ?, header_x5t = ?, token_blocked = ?, token_blocked_description = ? WHERE claims_jti = ?;",
            )
            .bind(model.claims_aud.clone())
            .bind(model.claims_exp)
            .bind(model.claims_iat)
            .bind(model.claims_iss.clone())
            .bind(model.claims_nbf)
            .bind(model.claims_sub.clone())
            .bind(model.header_typ.clone())
            .bind(model.header_alg.clone())
            .bind(model.header_cty.clone())
            .bind(model.header_jku.clone())
            .bind(model.header_kid.clone())
            .bind(model.header_x5u.clone())
            .bind(model.header_x5t.clone())
            .bind(model.token_blocked)
            .bind(model.token_blocked_description.clone())
            .bind(model.claims_jti.clone())
            .execute(&self.pool)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        Ok(done.rows_affected())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn validate(&self, claims_jti: &str) -> Result<Model, CommonError> {
        let now = Utc::now().timestamp();
        sqlx::query_as(
            "SELECT claims_aud, claims_exp, claims_iat, claims_iss, claims_jti, claims_nbf, claims_sub, header_typ, header_alg, header_cty, header_jku, header_kid, header_x5u, header_x5t, token_blocked, token_blocked_description FROM at WHERE ? < claims_exp AND claims_jti = ? AND token_blocked = FALSE",
        )
        .bind(now)
        .bind(claims_jti)
        .fetch_one(&self.pool)
        .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
        })
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn validate_by_claims_sub(&self, claims_sub: &str) -> Result<Model, CommonError> {
        let now = Utc::now().timestamp();
        sqlx::query_as(
            "SELECT claims_aud, claims_exp, claims_iat, claims_iss, claims_jti, claims_nbf, claims_sub, header_typ, header_alg, header_cty, header_jku, header_kid, header_x5u, header_x5t, token_blocked, token_blocked_description FROM at WHERE ? < claims_exp AND token_blocked = FALSE AND claims_sub = ?",
        )
        .bind(now)
        .bind(claims_sub)
        .fetch_one(&self.pool)
        .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err.into()
        })
    }
}
