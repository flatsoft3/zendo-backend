use crate::error::AppError;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub password: String,
    pub password_reset_token: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_id(db_pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
        match sqlx::query_as!(
            User,
            r#"
                SELECT id, 
                email, 
                full_name, 
                password, 
                password_reset_token, 
                is_active, 
                created_at, 
                updated_at
                
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
                SELECT id, 
                email, 
                full_name, 
                password, 
                password_reset_token, 
                is_active, 
                created_at, 
                updated_at
                
                FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_optional(db_pool)
        .await
    }

    pub async fn create(
        db_pool: &PgPool,
        id: Uuid,
        full_name: &str,
        email: &str,
        password: &str,
        password_reset_token: Option<&str>,
    ) -> Result<Self, AppError> {
        match sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (id,  email,  full_name,  password,  password_reset_token)
                
                VALUES ($1, $2, $3, $4, $5)

                RETURNING *
            "#,
            id,
            full_name,
            email,
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
}
