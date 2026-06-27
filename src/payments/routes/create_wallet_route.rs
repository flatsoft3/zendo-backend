use crate::common::enums::Currency;
use crate::models::wallet::Wallet;
use crate::{
    auth::extractor::AuthUser,
    common::{error::AppError, structs::ApiResponse},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateWalletPayload {
    #[validate(length(min = 4, message = "Name should be at least 4 characters"))]
    pub name: String,
    pub currency: Currency,
}

pub async fn create_wallet(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateWalletPayload>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::validation_error(e))?;

    // 1. Check if the name exists for the user
    if let Some(_) =
        Wallet::find_by_user_id_and_name(&state.db_pool, &auth_user.user_id, &payload.name).await?
    {
        return Err(AppError::bad_request(
            "Another wallet with the same name already exists.",
        ));
    };

    let wallet = Wallet::create(
        &state.db_pool,
        Uuid::new_v4(),
        &auth_user.user_id,
        payload.name.as_str(),
        payload.currency,
    )
    .await?;

    let response: ApiResponse<Wallet> =
        ApiResponse::success("Wallet created successfully", Some(wallet));

    Ok((StatusCode::CREATED, Json(response)))
}

// pub async fn validate_create_post(
//     payload: &CreatePostRequest,
//     db: &Database,
// ) -> Result<(), ValidationErrors> {
//     payload.validate()?;
//
//     if !category_repo::exists(db, payload.category_id).await? {
//         let mut errors = ValidationErrors::new();
//
//         errors.add(
//             "category_id",
//             ValidationError::new("does_not_exist"),
//         );
//
//         return Err(errors);
//     }
//
//     Ok(())
// }

//
// fn validate_even(value: &i32) -> Result<(), ValidationError> {
//     if value % 2 == 0 {
//         Ok(())
//     } else {
//         Err(ValidationError::new("not_even"))
//     }
// }
//
// #[derive(Validate)]
// struct Request {
//     #[validate(custom(function = "validate_even"))]
//     number: i32,
// }
