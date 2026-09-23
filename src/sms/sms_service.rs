use serde::Deserialize; 
use validator::Validate;

use crate::{common::{enums::{SmsChannel, SmsType}, error::AppError}, sms::sms_payloads::SendSmsResponse};

#[derive(Deserialize, Validate)]
pub struct SendSmsRequestPayload {
    wallet_id: Option<String>,
    #[validate(length(min = 1,  message = "At least one is recipient is required"))]
    recipients: Vec<String>,
    #[validate(length(min = 1,  message = "Message should not be empty"))]
    message: String,
    sms_type: SmsType,
    sms_channel: SmsChannel
}

pub struct SmsService {}

impl SmsService {

pub async fn send_sms(&self, payload: SendSmsRequestPayload)  -> Result<SendSmsResponse, AppError> {

    let (valid_recipients, invalid_recipients) : (Vec<&String> ,  Vec<&String>) = payload.recipients
        .iter()
        .partition(|r| Self::validate_recipient(r).is_some());
        

    if valid_recipients.is_empty() {
        return Err(AppError::bad_request( "All recipients are invalid"));
    }

    Ok(SendSmsResponse::InsufficientBalance)
}


fn validate_recipient(recipient: &str) -> Option<&str> {

     (recipient.len() == 13 && recipient.bytes().all(|b| b.is_ascii_digit()))
        .then_some(recipient)

}

}