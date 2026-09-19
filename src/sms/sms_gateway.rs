use crate::{
    common::{
        enums::{SmsChannel, SmsType},
        error::AppError,
    },
    sms::sms_payloads::SendSmsResponse,
};
use async_trait::async_trait;


#[async_trait]
pub trait SmsGateway {
    async fn send_sms(
        &self,
        recipients: Vec<String>,
        message: &str,
        sender_id: &str,
        sms_type: SmsType,
        sms_channel: SmsChannel,
    ) -> Result<SendSmsResponse, AppError>;
}
