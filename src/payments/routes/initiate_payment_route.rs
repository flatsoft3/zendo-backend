// use std::collections::HashMap;

use crate::{
    auth::extractor::AuthUser,
    common::{self, enums::Currency, error::AppError, structs::ApiResponse},
    models::payment::Payment,
    payments::gateways::{
        korapay::korapay::KorapayGateway,
        payment_gateway::{
            self, CardPaymentGateway, InitiateCardPaymentRequest, InitiateCardPaymentResponse,
            PaymentGateway,
        },
    },
    state::AppState,
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct InitiatePaymentPayload {
    #[validate(range(min = 200, message = "Amount must be at least 200"))]
    pub amount: u64,
    pub gateway: crate::common::enums::PaymentGateway,
    pub wallet_id: Uuid,
    pub currency: Currency,
}

pub async fn initiate_payment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request_payload): Json<InitiatePaymentPayload>,
) -> Result<impl IntoResponse, AppError> {
    request_payload
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
                request_payload.amount.into(),
                request_payload.currency,
                &reference,
                None,
                Some(request_payload.gateway.as_ref()),
            )
            .await
            {
                Err(e) => Err(e),

                Ok(payment) => {
                    //payment created, lets initiate it
                    let payment_gateway: Box<dyn CardPaymentGateway> = match request_payload.gateway
                    {
                        common::enums::PaymentGateway::Korapay => {
                            Box::new(KorapayGateway::new(state.config.korapay))
                        }
                        _ => Box::new(KorapayGateway::new(state.config.korapay)),
                    };

                    let request = InitiateCardPaymentRequest {
                        amount: request_payload.amount,
                        currency: request_payload.currency,
                        reference: reference,
                        customer_bears_charges: false,
                        email: auth_user.email,
                        redirect_url: state.config.payment.redirect_url,
                        payer_name: Some(auth_user.full_name),
                    };

                    match payment_gateway.initiate_card_payment(request).await {
                        Err(e) => Err(e),
                        Ok(InitiateCardPaymentResponse::Initiated {
                            reference,
                            checkout_url,
                            gateway_reference,
                        }) => {
                            // save this to initiated payment table

                            let mut payload = serde_json::Map::new();

                            payload.insert("amount".to_string(), json!(&payment.amount));
                            payload.insert("reference".to_string(), json!(reference));
                            payload.insert("checkout_url".to_string(), json!(checkout_url));
                            payload.insert("gateway".to_string(), json!(request_payload.gateway));
                            payload
                                .insert("gateway_reference".to_string(), json!(gateway_reference));

                            let json_string = serde_json::to_string(&payload).unwrap();

                            let response: ApiResponse<String> = ApiResponse::success(
                                "Payment created",
                                Some(
                                    json_string
                                        // HashMap::from([
                                        //     ("key1", "value1"),
                                        //     ("key2", "value2"),
                                        //     ("key2", payment.amount),
                                        // ])
                                        .into(),
                                ),
                            );

                            Ok((StatusCode::OK, Json(response)))
                        }
                        Ok(InitiateCardPaymentResponse::GatewayError { message }) => {
                            Err(AppError::bad_gateway(format!("Gateway Error: {}", message)))
                        }

                        Ok(_) => Err(AppError::internal(format!("Failed to initiate payment"))),
                    }
                }
            }
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/payments/initiate", post(initiate_payment))
}
