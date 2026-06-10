use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

//requests


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
    // customer: Customer,
    kyc: Kyc
}



//responses



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