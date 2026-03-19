use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize)]
pub struct UserCreatedResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserCreatedResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            created_at: user.created_at,
        }
    }
}

impl UserCreatedResponse {
    pub fn get_me_from_user() -> Self {
        let usr: User = User {
            id: Uuid::new_v4(),
            email: "myemail@gmail.com".to_string(),
            full_name: "Sani Ahmad Badawa".to_string(),
            password: "my_password".to_string(),
            password_reset_token: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        usr.into()
    }
}


#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub user_info: UserCreatedResponse
}