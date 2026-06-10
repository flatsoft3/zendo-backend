use crate::common::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait CardPaymentGateway {
    async fn initiate_card_payment(
        &self,
        request: InitiateCardPaymentRequest,
    ) -> Result<InitiateCardPaymentResponse, AppError>;

    async fn check_payment_status(
        &self,
        reference: String,
    ) -> Result<PaymentStatusResponse, AppError>;
}

pub enum PaymentStatusResponse {
    SuccessfulPayment {
        amount_paid: u64,
        gateway_charges: u64,
        gateway_reference: String,
        paid_at: String,
        payment_method: Option<String>,
    },
    PendingPayment {
        gateway_response: String,
    },
    ReferenceNotFound,
}

pub struct InitiateCardPaymentRequest {
    pub amount: u64,
    pub currency: String,
    pub reference: String,
    pub charges_bearer: bool,
    pub email: String,
    pub redirect_url: String,
    pub payer_name: Option<String>,
}

pub enum InitiateCardPaymentResponse {
    Initiated {
        reference: String,
        checkout_url: String,
        gateway_reference: Option<String>,
    },
    GatewayError {
        message: String,
    },
    JsonParseError {
        message: String,
    },
    ApplicationError {
        message: String,
    },
}

pub trait VirtualAccountProvider {}

pub trait PaymentGateway {
    fn name() -> &'static str;
}
