use crate::{
    auth::extractor::AuthUser,
    common::{error::AppError, structs::ApiResponse},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::models::wallet::Wallet;

pub async fn get_wallets(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let wallets = Wallet::find_by_user_id(&state.db_pool, auth_user.user_id).await?;

    let response: ApiResponse<Vec<Wallet>> = ApiResponse::success("User wallets", Some(wallets));

    Ok((StatusCode::OK, Json(response)))
}
