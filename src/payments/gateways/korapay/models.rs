use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

//requests
#[derive(Debug, Serialize)]
pub struct Customer {
   pub name: String,
   pub email: Option<String>
}

#[derive(Debug, Serialize)]
pub struct Kyc {
  pub bvn: String,
  pub nin: Option<String>
}

#[derive(Debug, Serialize)]
pub struct CreateVirtualAccountRequest {
    account_name: String,
    account_reference: String,
    permanent: bool,
    bank_code: String,
    customer: Customer,
    kyc: Kyc
}

#[derive(Debug, Serialize)]
pub struct InitializePaymentRequest {
    pub amount: u64,
    pub redirect_url: String,
    pub currency: String,
    pub reference: String,
    pub channels: Vec<String>,
    pub customer: Customer,
    pub merchant_bears_cost: bool,
} 


//responses

#[derive(Debug, Serialize, Deserialize)]
pub struct InitiatePaymentResponse {
    pub status: bool,
    pub message: String,
    pub code: Option<String>,
    pub data: Option<InitiatePaymentData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitiatePaymentData {
    pub reference: String,
    pub checkout_url: String,
} 

#[derive(Debug, Deserialize)]
pub struct WebhookNotificationPayload {
    pub event: String,
    pub data: WebhookNotificationData,
}

#[derive(Debug, Deserialize)]
pub struct WebhookNotificationData {
    pub reference: String,
    pub payment_reference: String,
    pub currency: String,
    pub amount: Decimal,
    pub fee: Decimal,
    pub payment_method: String,
    pub status: String,
  //  pub virtual_bank_account_details: Option<VirtualBankAccountDetails>,
    pub transaction_date: String,
}

// #[derive(Debug, Deserialize)]
// pub struct VirtualBankAccountDetails {
//     pub virtual_bank_account: VirtualBankAccount,
// }

// #[derive(Debug, Deserialize)]
// pub struct VirtualBankAccount {
//     pub account_name: String,
//     pub account_number: String,
// }