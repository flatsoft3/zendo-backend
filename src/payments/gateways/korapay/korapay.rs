use crate::{
    config::KorapayConfig,
    error::AppError,
    payments::gateways::payment_gateway::{
        CardPaymentGateway, InitiateCardPaymentRequest, InitiateCardPaymentResponse,
        PaymentGateway, PaymentStatusResponse,
    },
    state::AppState,
};
use async_trait::async_trait;

pub struct Korapay {
    config: KorapayConfig,
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
