use crate::common::error::AppError;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub phone_number: String,
    pub company_name: Option<String>,
    pub rc_number: Option<String>,
    pub tax_id: Option<String>,
    pub company_address: Option<String>,
    pub password: String,
    pub password_reset_token: Option<String>,
    pub kyc_tier: i16,
    pub kyc_verified_at: Option<DateTime<Utc>>,
    pub account_status: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_id(db_pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
        match sqlx::query_as!(
            User,
            r#"
                SELECT *
                
                FROM users
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db_pool)
        .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::from(e)),
        }

        // found_user.ok_or(AppError::NotFound)
    }

    pub async fn find_by_email(db_pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
       sqlx::query_as!(
            User,
            r#"
                SELECT *
                
                FROM users
                WHERE LOWER(email) = LOWER($1)
            "#,
            email
        )
        .fetch_optional(db_pool)
        .await
    }

   pub async fn create(
    db_pool: &PgPool,
    id: Uuid,
    email: &str,
    first_name: &str,
    middle_name: Option<&str>,
    last_name: &str,
    phone_number: &str,
    password: &str,
    password_reset_token: Option<&str>,
) -> Result<Self, AppError> {
    match sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            id,
            email,
            first_name,
            middle_name,
            last_name,
            phone_number,
            password,
            password_reset_token
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8
        )
        RETURNING *
        "#,
        id,
        email,
        first_name,
        middle_name,
        last_name,
        phone_number,
        password,
        password_reset_token,
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(new_user) => Ok(new_user),
        Err(e) => Err(AppError::from(e)),
    }
}

    pub async fn update_company_details(
        db_pool: &PgPool,
        user_id: Uuid,
        company_name: &str,
        rc_number: &str,
        tax_id: Option<&str>,
        company_address: &str,
    ) -> Result<Self, AppError> {
        match sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                company_name    = $2,
                rc_number       = $3,
                tax_id          = $4,
                company_address = $5,
                updated_at      = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
            user_id,
            company_name,
            rc_number,
            tax_id,
            company_address,
        )
        .fetch_one(db_pool)
        .await
        {
            Ok(updated_user) => Ok(updated_user),
            Err(sqlx::Error::RowNotFound) => Err(AppError::not_found("User not found")),
            Err(e) => Err(AppError::from(e)),
        }
    }
}
