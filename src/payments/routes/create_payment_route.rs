use crate::{
    auth::extractor::AuthUser,
    common::{enums::Currency, error::AppError, structs::ApiResponse},
    models::payment::Payment,
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreatePaymentPayload {
    #[validate(range(min = 200, message = "Amount must be at least 200"))]
    pub amount: u64,
    pub wallet_id: Uuid,
    pub currency: Currency,
    pub description: Option<String>,
}

pub async fn create_payment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreatePaymentPayload>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::validation_error(e))?;

    let reference = Payment::generate_reference(&state.db_pool).await?;

    let payment = Payment::create(
        &state.db_pool,
        Uuid::new_v4(),
        auth_user.user_id,
        payload.wallet_id,
        payload.amount.into(),
        payload.currency,
        &reference,
        payload.description.as_deref(),
        None, // no gateway yet at creation time
    )
    .await?;

    let response: ApiResponse<Payment> =
        ApiResponse::success("Payment created successfully", Some(payment));

    Ok((StatusCode::CREATED, Json(response)))
}
