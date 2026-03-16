use crate::{
    common::{structs::ApiResponse, util},
    dtos::{requests::CreateUserRequest, responses::UserCreatedResponse},
    error::AppError,
    state::AppState,
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::models::user::User;
use uuid::Uuid;

async fn find_by_id(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let user_id = Uuid::nil();

    match User::find_by_id(&state.db_pool, user_id).await {
        Ok(Some(user)) => {
            let response: ApiResponse<UserCreatedResponse> =
                ApiResponse::success("User was found", Some(user.into()));
            Ok((StatusCode::OK, Json(response)))
        }

        Ok(None) => Err(AppError::not_found("user not found")),
        Err(e) => Err(e), // AppError::internal(format!("Error getting user:")))
    }
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    match User::find_by_email(&state.db_pool, &payload.email).await {
        Err(e) => Err(e.into()),
        Ok(Some(_)) => Err(AppError::bad_request("User already exists")),
        Ok(None) => {
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
                Ok(new_user) => {
                    let response: ApiResponse<UserCreatedResponse> =
                        ApiResponse::success("User was created successfully", Some(new_user.into()));

                    Ok((StatusCode::OK, Json(response)))
                }
                Err(e) => Err(e),
            }
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/find-by-id", get(find_by_id))
        .route("/users/create", post(create))
}
