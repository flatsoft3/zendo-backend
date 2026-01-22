use crate::state::AppState;
use axum::{extract::State, response::IntoResponse};

use crate::models::user::User;
use uuid::Uuid;

pub async fn find_by_id(State(state): State<AppState>) -> impl IntoResponse {
    let user_id = Uuid::nil();

    match User::find_by_id(&state.db_pool, user_id).await {
        Ok(Some(user)) => format!(
            "User found with name as: {} and email as : {}",
            user.full_name, user.email
        ),
        Ok(None) => format!("user not found"),
        Err(e) => format!("Error getting user: {}", e.to_string()),
    }
}
