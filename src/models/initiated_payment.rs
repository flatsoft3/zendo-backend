use crate::common::error::AppError;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct InitiatedPayment {
    pub id: Uuid,
    pub payment_reference: String,
    pub amount: sqlx::types::Decimal,
    pub gateway: String,
    pub gateway_reference: Option<String>,
    pub checkout_url: Option<String>, 
    pub created_at: DateTime<Utc>,
}

impl InitiatedPayment {
    pub async fn find_by_reference_and_gateway(
        db_pool: &PgPool,
        payment_reference: &str,
        gateway: &str,
    ) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            InitiatedPayment,
            r#"
            SELECT *
            FROM initiated_payments
            WHERE payment_reference = $1
              AND gateway = $2
            "#,
            payment_reference,
            gateway,
        )
        .fetch_optional(db_pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn create(
        db_pool: &PgPool,
        id: Uuid,
        payment_reference: &str,
        amount: sqlx::types::Decimal,
        gateway: &str,
        gateway_reference: Option<&str>,
        checkout_url: Option<&str>,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            InitiatedPayment,
            r#"
            INSERT INTO initiated_payments (
                id,
                payment_reference,
                amount,
                gateway,
                gateway_reference,
                checkout_url
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            id,
            payment_reference,
            amount,
            gateway,
            gateway_reference,
            checkout_url
        )
        .fetch_one(db_pool)
        .await
        .map_err(AppError::from)
    }
}
