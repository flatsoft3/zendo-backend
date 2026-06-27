use crate::{
    common::error::AppError,
    config::KorapayConfig,
    payments::gateways::payment_gateway::{
        CardPaymentGateway, InitiateCardPaymentRequest, InitiateCardPaymentResponse,
        PaymentGateway, PaymentStatusResponse,
    },
};
use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use serde_json;

pub struct KorapayGateway {
    config: KorapayConfig,
}

#[derive(Debug, Serialize)]
struct Customer {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
struct InitializePaymentRequest {
    pub amount: u64,
    pub redirect_url: String,
    pub currency: String,
    pub reference: String,
    pub channels: Vec<String>,
    pub customer: Customer,
    pub merchant_bears_cost: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitiatePaymentResponse {
    pub status: bool,
    pub message: String,
    pub code: Option<String>,
    pub data: Option<InitiatePaymentData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitiatePaymentData {
    pub reference: String,
    pub checkout_url: String,
}

impl KorapayGateway {
  pub  fn new(config: KorapayConfig) -> Self {
        Self { config }
    }
}

impl PaymentGateway for KorapayGateway {
    fn name() -> &'static str {
        "Korapay"
    }
}

#[async_trait]
impl CardPaymentGateway for KorapayGateway {
    async fn initiate_card_payment(
        &self,
        request: InitiateCardPaymentRequest,
    ) -> Result<InitiateCardPaymentResponse, AppError> {
        let initiate_payment_payload = InitializePaymentRequest {
            amount: request.amount,
            redirect_url: request.redirect_url,
            currency: request.currency.to_string(),
            reference: request.reference,
            channels: self
                .config
                .allowed_channels
                .split(",")
                .map(|x| x.to_string())
                .collect(),
            customer: Customer {
                name: request.payer_name,
                email: request.email,
            },
            merchant_bears_cost: true,
        };

        tracing::debug!(
            payload = %serde_json::to_string(&initiate_payment_payload).unwrap_or_default(),
            "Sending initiate card payment request to Korapay"
        );

        let http_client = reqwest::Client::new();
        let http_response = http_client
            .post(&self.config.initiate_card_payment_url)
            .json(&initiate_payment_payload)
            .header(
                "Authorization",
                format!("Bearer {}", &self.config.secret_key),
            )
            .send()
            .await
            .unwrap();

        let response_bytes = http_response.bytes().await?;

        let gateway_response: InitiatePaymentResponse =
            serde_json::from_slice(&response_bytes).map_err(|e| {
                let raw = String::from_utf8_lossy(&response_bytes);
                tracing::error!(
                    error = %e,
                    raw_body = %raw,
                    "Failed to decode Korapay response"
                );
                AppError::bad_gateway(format!("Failed to decode gateway response: {}", e))
            })?;

        match gateway_response.data {
            None => Ok(InitiateCardPaymentResponse::GatewayError {
                message: gateway_response.message,
            }),

            Some(data) => Ok(InitiateCardPaymentResponse::Initiated {
                reference: data.reference,
                checkout_url: data.checkout_url,
                gateway_reference: None,
            }),
        }
    }

    async fn check_payment_status(
        &self,
        _reference: String,
    ) -> Result<PaymentStatusResponse, AppError> {
        Ok(PaymentStatusResponse::ReferenceNotFound)
    }
}
