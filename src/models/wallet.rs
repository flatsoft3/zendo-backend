use crate::common::enums::Currency;
use crate::common::error::AppError;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Wallet {
    pub async fn create(
        db_pool: &PgPool,
        id: Uuid,
        user_id: &Uuid,
        name: &str,
        currency: Currency,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            Wallet,
            r#"
            INSERT INTO wallets (
                id, 
                user_id, 
                name,  
                currency
            )
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            id,
            user_id,
            name,
            currency.as_ref()
        )
        .fetch_one(db_pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_user_id(db_pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Wallet,
            r#"
            SELECT *
            FROM wallets
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_all(db_pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_user_id_and_name(
        db_pool: &PgPool,
        user_id: &Uuid,
        name: &str,
    ) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Wallet,
            r#"
            SELECT * 
            FROM  wallets 
            WHERE user_id = $1 AND name = $2
            "#,
            user_id,
            name
        )
        .fetch_optional(db_pool)
        .await
        .map_err(AppError::from)
    }



    pub async fn find_by_id(db_pool: &PgPool, id: &Uuid) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Wallet,
            r#"
            SELECT *
            FROM wallets
            WHERE id = $1
            "#,
            id,
        )
            .fetch_optional(db_pool)
            .await
            .map_err(AppError::from)
    }
}
