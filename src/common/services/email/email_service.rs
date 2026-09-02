use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType, transport::smtp::{authentication::Credentials, client::{Tls, TlsParameters}}
};

use crate::{common::error::AppError, config::{SmtpConfig, TlsStrategy}};
use validator::ValidateEmail;

#[derive(Debug)]
pub enum SendEmailStatus {
    Sent,
    SmtpError { message: String },
    InvalidEmail,
    Failed { error: String },
}

#[derive(Debug)]
pub struct EmailPayload {
    pub to: String,
    pub subject: String,
    pub body: String, // HTML or plain text
}

#[derive(Clone)]
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl EmailService {
    pub fn new(config: SmtpConfig) -> Result<Self, AppError> {
        let credentials = Credentials::new(config.username, config.password);

         let mailer = match config.tls_strategy.unwrap_or_default() {
            TlsStrategy::Plain => {
                AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?
                    .port(config.port)
                    .credentials(credentials)
                    .tls(Tls::None)
                    .build()
            }
            TlsStrategy::StartTls => {
                // For STARTTLS (port 587), use starttls_relay
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)?
                    .port(config.port)
                    .credentials(credentials)
                    .build()
            }
            TlsStrategy::ImplicitTls => {
                // For implicit TLS (port 465), use relay with Tls::Wrapper
                let tls_parameters = TlsParameters::new(config.host.clone())?;
                
                AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?
                    .port(config.port)
                    .credentials(credentials)
                    .tls(Tls::Wrapper(tls_parameters))
                    .build()
            }
        };


        Ok(Self {
            mailer,
            from: config.from,
        })
    }

    pub async fn send_mail(&self, payload: EmailPayload) -> Result<SendEmailStatus, AppError> {
        if !&payload.to.validate_email() {
            return Ok(SendEmailStatus::InvalidEmail);
        }

        let email = Message::builder()
            .from(self.from.parse()?)
            .to(payload.to.parse()?)
            .subject(payload.subject)
            .header(ContentType::TEXT_HTML)
            .body(payload.body)?;

        match self.mailer.send(email).await {
            Ok(_) => Ok(SendEmailStatus::Sent),

            Err(e) => Ok(SendEmailStatus::SmtpError { message: e.to_string() }),
        }
    }
}
