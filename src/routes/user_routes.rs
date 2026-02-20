use crate::{
    common::util, dtos::{requests::CreateUserRequest, responses::UserCreatedResponse}, error::AppError, state::AppState
};
use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};

use crate::models::user::User;
use uuid::Uuid;

async fn find_by_id(State(state): State<AppState>) -> impl IntoResponse {
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

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserCreatedResponse>, AppError> {
    match User::create(
        &state.db_pool,
        Uuid::new_v4(),
        &payload.full_name,
        &payload.email,
        &util::hash_password(&payload.password),
        None,
    )
    .await
    {
        Ok(new_user) => Ok(Json(new_user.into())),
        Err(e) => Err(e),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/find-by-id", get(find_by_id))
        .route("/users/create", post(create))
}
