use crate::{
    auth::extractor::AuthUser,
    common::{enums::PaymentGateway, error::AppError, structs::ApiResponse},
    models::{initiated_payment::InitiatedPayment, payment::Payment},
    payments::gateways::{
        korapay::korapay::KorapayGateway,
        payment_gateway::{
            CardPaymentGateway, InitiateCardPaymentRequest, InitiateCardPaymentResponse,
        },
    },
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use crate::common::enums::PaymentStatus;

#[derive(Deserialize, Validate)]
pub struct InitiatePaymentPayload {
    #[validate(length(min = 1, message = "Reference is required"))]
    pub reference: String,
    pub gateway: PaymentGateway,
}

pub async fn initiate_payment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<InitiatePaymentPayload>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::validation_error(e))?;

    // 1. Find the payment record
    let payment = match Payment::find_by_reference(&state.db_pool, &payload.reference).await? {
        None => return Err(AppError::not_found("Payment not found")),
        Some(p) => p,
    };

    // 2. Check if already paid
    if payment.status == PaymentStatus::Success.as_ref() {
        return Err(AppError::bad_request(
            "This payment has already been completed",
        ));
    }

    let gateway_str = payload.gateway.as_ref();

    // 3. Check if already initiated for this gateway — return existing record if so
    if let Some(existing) =
        InitiatedPayment::find_by_reference_and_gateway(&state.db_pool, &payload.reference, gateway_str)
            .await?
    {
        let response: ApiResponse<serde_json::Value> = ApiResponse::success(
            "Payment already initiated",
            Some(json!({
                "payment_id": payment.id,
                "reference": existing.payment_reference,
                "amount": existing.amount,
                "gateway": existing.gateway,
                "gateway_reference": existing.gateway_reference,
                "checkout_url": existing.checkout_url,
                "created_at": existing.created_at,
            })),
        );
        return Ok((StatusCode::OK, Json(response)));
    }

    // 4. Initiate via gateway
    let gateway: Box<dyn CardPaymentGateway + Send + Sync> = match payload.gateway {
        PaymentGateway::Korapay => Box::new(KorapayGateway::new(state.config.korapay)),
    };

    let gateway_request = InitiateCardPaymentRequest {
        amount: payment.amount.try_into().unwrap_or(0),
        currency: payment.currency.parse().map_err(|_| {
            AppError::internal(format!("Invalid currency on payment: {}", payment.currency))
        })?,
        reference: payment.reference.clone(),
        customer_bears_charges: false,
        email: auth_user.email,
        redirect_url: state.config.payment.redirect_url,
        payer_name: Some(auth_user.full_name),
    };

    match gateway.initiate_card_payment(gateway_request).await? {
        InitiateCardPaymentResponse::Initiated {
            reference: _,
            checkout_url,
            gateway_reference,
        } => {
            // 5. Save to initiated_payments
            let initiated = InitiatedPayment::create(
                &state.db_pool,
                Uuid::new_v4(),
                &payment.reference,
                payment.amount,
                gateway_str,
                gateway_reference.as_deref(),
                Some(&checkout_url),
            )
            .await?;

            let response: ApiResponse<serde_json::Value> = ApiResponse::success(
                "Payment initiated successfully",
                Some(json!({
                    "payment_id": payment.id,
                    "reference": payment.reference,
                    "amount": payment.amount,
                    "gateway": initiated.gateway,
                    "gateway_reference": initiated.gateway_reference,
                    "checkout_url": initiated.checkout_url,
                    "created_at": initiated.created_at,
                })),
            );

            Ok((StatusCode::OK, Json(response)))
        }

        InitiateCardPaymentResponse::GatewayError { message } => {
            Err(AppError::bad_gateway(format!("Gateway error: {}", message)))
        }

        _ => Err(AppError::internal("Unexpected response from payment gateway")),
    }
}
