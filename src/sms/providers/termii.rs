// use std::format;

use async_trait::async_trait;
use reqwest::StatusCode;
use rust_decimal::Decimal;
use serde_json::json;

use crate::{
    common::{
        enums::{SmsChannel, SmsType},
        error::AppError,
    },
    config::TermiiSmsConfig,
    sms::{sms_gateway::SmsGateway, sms_payloads::SendSmsResponse},
};

pub struct TermiiSms {
    config: TermiiSmsConfig,
}

impl TermiiSms {
    pub fn new(config: TermiiSmsConfig) -> Self {
        Self { config }
    }
}

#[derive(serde::Deserialize)]
struct TermiiSmsSentResponse {
    balance: Option<Decimal>,
    code: String,
    message: String,
    message_id: String,
}

#[derive(serde::Deserialize)]
struct FieldError {
    field: String,
    message: String,
    #[serde(rename = "rejectedValue")]
    rejected_value: String,
}

#[derive(serde::Deserialize)]
struct TermiiSmsErrorResponse {
    status: i32,
    error: String,
    message: String,
    #[serde(rename = "fieldErrors")]
    field_errors: Option<Vec<FieldError>>,
}


#[async_trait]
impl SmsGateway for TermiiSms {
    async fn send_sms(
        &self,
        recipients: Vec<String>,
        message: &str,
        sender_id: &str,
        sms_type: SmsType,
        sms_channel: SmsChannel,
    ) -> Result<SendSmsResponse, AppError> {
        let request_payload = json!({
            "api_key" : &self.config.api_key,
            "from" : &self.config.default_sender_id,
            "sms" : message,
            "type" : sms_type.to_string().to_lowercase(),
            "channel" : sms_channel.to_string().to_lowercase(),
            "to" : recipients
        });

        let http_client = reqwest::Client::new();
        let http_response = http_client
            .post(&self.config.send_sms_url)
            .json(&request_payload)
            .send()
            .await
            .unwrap();

        match http_response.status() {
            StatusCode::OK => {
                let response_bytes = http_response.bytes().await?;

                let gateway_response: TermiiSmsSentResponse =
                    serde_json::from_slice(&response_bytes).map_err(|e| {
                        let raw = String::from_utf8_lossy(&response_bytes);
                        tracing::error!(
                            error = %e,
                            raw_body = %raw,
                            "{}", format!("Failed to decode termii sms response: {}", raw)
                        );
                        AppError::bad_gateway(format!("Failed to decode gateway response: {}", e))
                    })?;

                Ok(SendSmsResponse::Sent {
                    units_used: 0.0,
                    sent_to: recipients,
                    not_sent_to: None,
                    provider_reference: gateway_response.message_id,
                })
            }

            x => {
                let response_bytes = http_response.bytes().await?;

                let gateway_response: TermiiSmsErrorResponse =
                    serde_json::from_slice(&response_bytes).map_err(|e| {
                        let raw = String::from_utf8_lossy(&response_bytes);
                        tracing::error!(
                            error = %e,
                            raw_body = %raw,
                            "{}", format!("Failed to decode termii sms response: {}", raw)
                        );
                        AppError::bad_gateway(format!("Failed to decode gateway response: {}", e))
                    })?;

                Ok(SendSmsResponse::Failed {
                    error: gateway_response.message,
                })
            }
        }
    }
}
