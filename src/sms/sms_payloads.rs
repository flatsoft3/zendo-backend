

#[derive(Debug)]
pub enum SendSmsResponse {
    Sent {
        units_used: f64,
        sent_to: Vec<String>,
        not_sent_to: Option<Vec<String>>,
        provider_reference: String
    },
    ProviderError { message: String },
    InvalidPhoneNumber,
    InsufficientBalance,
    Failed { error: String },
}
