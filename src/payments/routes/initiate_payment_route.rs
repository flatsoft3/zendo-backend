use crate::{
    auth::extractor::AuthUser,
    common::{
        enums::{Currency, PaymentGateway},
        error::AppError,
        structs::ApiResponse,
    },
    models::payment::Payment,
    state::AppState,
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct InitiatePaymentPayload {
    #[validate(range(min = 200, message = "Amount must be at least 200"))]
    pub amount: u64,
    pub gateway: PaymentGateway,
    pub wallet_id: Uuid,
    pub currency: Currency,
}

pub async fn initiate_payment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<InitiatePaymentPayload>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::validation_error(e))?;

    match Payment::generate_reference(&state.db_pool).await {
        Err(e) => Err(e),

        Ok(reference) => {
            match Payment::create(
                &state.db_pool,
                Uuid::new_v4(),
                auth_user.user_id,
                Uuid::new_v4(),
                payload.amount.into(),
                payload.currency,
                &reference,
                None,
            )
            .await
            {
                Err(e) => Err(e),

                Ok(payment) => {
                    let response: ApiResponse<Payment> =
                        ApiResponse::success("Payment created", Some(payment.into()));

                    Ok((StatusCode::OK, Json(response)))
                }
            }
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/payments/initiate", post(initiate_payment))
}
