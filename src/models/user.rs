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
    pub async fn find_by_id(db_pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
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
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db_pool)
        .await
    }
}
