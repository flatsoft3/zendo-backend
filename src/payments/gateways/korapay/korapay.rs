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

pub struct Korapay {
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

impl Korapay {
    fn new(config: KorapayConfig) -> Self {
        Self { config }
    }
}

impl PaymentGateway for Korapay {
    fn name() -> &'static str {
        "Korapay"
    }
}

#[async_trait]
impl CardPaymentGateway for Korapay {
    async fn initiate_card_payment(
        &self,
        request: InitiateCardPaymentRequest,
    ) -> Result<InitiateCardPaymentResponse, AppError> {
        let initiate_payment_payload = InitializePaymentRequest {
            amount: request.amount,
            redirect_url: request.redirect_url,
            currency: request.currency,
            reference: request.reference,
            channels: self
                .config
                .allowed_channels
                .split(",")
                .map(|x| x.to_string())
                .collect(),
            customer: Customer {
                name: None,
                email: request.email,
            },
            merchant_bears_cost: true,
        };

        // let http_client = reqwest::Client::new();
        // let http_response = http_client
        //     .post(self.config.initiate_card_payment_url)
        //     .json(&initiate_payment_payload)
        //     .header(
        //         "Authorization",
        //         format!("Bearer {}", self.config.secret_key),
        //     )
        //     .send()
        //     .await
        //     .unwrap();

        // let body = http_response.text().await?;

        Ok(InitiateCardPaymentResponse::ApplicationError {
            message: "Failed".to_string(),
        })
    }

    async fn check_payment_status(
        &self,
        reference: String,
    ) -> Result<PaymentStatusResponse, AppError> {
        Ok(PaymentStatusResponse::ReferenceNotFound)
    }
}
