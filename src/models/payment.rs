use crate::common::{enums::{Currency, PaymentStatus}, error::AppError};
use chrono::{DateTime, Utc};
use rand::Rng;
use rust_decimal::Decimal;
use serde::Serialize;
// use rand::RngExt;
use sqlx::PgPool;
use uuid::Uuid;

// use rand::Rng;

#[derive(Debug, Serialize)]
pub struct Payment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: sqlx::types::Decimal,
    pub currency: String,
    pub reference: String,
    pub status: String,
    pub description: Option<String>,
    pub gateway_reference: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Payment {
    /// Insert a new payment transaction. Status defaults to 'pending' in the DB.
    pub async fn create(
        db_pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        wallet_id: Uuid,
        amount: sqlx::types::Decimal,
        currency: Currency,
        reference: &str,
        description: Option<&str>,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            Payment,
            r#"
            INSERT INTO payments (
                id, 
                user_id, 
                wallet_id, 
                amount, 
                currency,
                reference, 
                description
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            id,
            user_id,
            wallet_id,
            amount,
            currency.as_ref(),
            reference,
            description
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(ref db_err)
                if db_err.constraint() == Some("payments_reference_key") =>
            {
                AppError::bad_request(format!("Reference '{}' already exists", reference))
            }
            _ => AppError::from(e),
        })
    }

    /// Fetch a single transaction by its unique reference.
    pub async fn find_by_reference(
        db_pool: &PgPool,
        reference: &str,
    ) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Payment,
            r#"
            SELECT *
            FROM payments
            WHERE reference = $1
            "#,
            reference,
        )
        .fetch_optional(db_pool)
        .await
        .map_err(AppError::from)
    }

    /// Fetch a single transaction by its internal UUID.
    // pub async fn find_by_id(db_pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
    //     sqlx::query_as!(
    //         Payment,
    //         r#"
    //         SELECT
    //             id, user_id, wallet_id, amount, currency, reference,
    //             status::TEXT AS "status!",
    //             description, gateway_reference, paid_at, created_at
    //         FROM payments
    //         WHERE id = $1
    //         "#,
    //         id,
    //     )
    //     .fetch_optional(db_pool)
    //     .await
    //     .map_err(AppError::from)
    // }

    /// Fetch all transactions belonging to a user, newest first.
    // pub async fn find_by_user(db_pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, AppError> {
    //     sqlx::query_as!(
    //         Payment,
    //         r#"
    //         SELECT
    //             id, user_id, wallet_id, amount, currency, reference,
    //             status::TEXT AS "status!",
    //             description, gateway_reference, paid_at, created_at
    //         FROM payments
    //         WHERE user_id = $1
    //         ORDER BY created_at DESC
    //         "#,
    //         user_id,
    //     )
    //     .fetch_all(db_pool)
    //     .await
    //     .map_err(AppError::from)
    // }

    /// Update status (and optionally gateway_reference / paid_at) after a gateway callback.
    pub async fn update_status(
        db_pool: &PgPool,
        id: Uuid,
        amount: Decimal,
        status: PaymentStatus,
        gateway_reference: Option<&str>,
        paid_at: Option<DateTime<Utc>>,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            Payment,
            r#"
            UPDATE payments
            SET
                amount            = $2,
                status            = $3,
                gateway_reference = $4,
                paid_at           = $5
            WHERE id = $1
            RETURNING *
            "#,
            id,
            amount,
            status.as_ref(),
            gateway_reference,
            paid_at,
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::not_found("Payment transaction not found"),
            _ => AppError::from(e),
        })
    }

   pub async fn generate_reference(db_pool: &PgPool) -> Result<String, AppError> {
        let mut attempts = 0;

        loop {
            let new_reference = {
                let mut rng = rand::thread_rng();
                format!("ZNDPT{}", rng.gen_range(10_000_000u64..99_999_999u64))
            }; // rng is dropped here, before the .await

            match Payment::find_by_reference(db_pool, &new_reference).await? {
                Some(_) => {
                    attempts += 1;

                    if attempts >= 50 {
                        return Err(AppError::internal(
                            "Unable to generate payment reference [Code: MAXRC]",
                        ));
                    }
                }

                None => return Ok(new_reference),
            }
        }
    }
}
