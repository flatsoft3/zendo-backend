use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Payment {
    pub minimum_amount_to_initiate: u64,
    pub redirect_url: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct KorapayConfig {
    pub allowed_channels : String,
    pub create_virtual_account_url: String,
    pub initiate_card_payment_url: String,
    pub check_payment_status_url: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum TlsStrategy {
    Plain,
    StartTls,
    ImplicitTls,
}

impl Default for TlsStrategy {
    fn default() -> Self {
        Self::StartTls  // Most common default
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    pub host:     String,
    pub port:     u16,
    pub username: String,
    pub password: String,
    pub from:     String,
    pub tls_strategy: Option<TlsStrategy>
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig{
    pub app_name: String,
    pub app_env: String,
    pub app_port: u16,
    pub database_url: String,
    pub app_url: String,
    pub jwt_user_key: String,
    pub jwt_expiry: u32,
    pub korapay: KorapayConfig,
    pub payment: Payment,
    pub smtp_config: SmtpConfig,
    pub redis_url: String
}

impl AppConfig {
    pub fn load() -> Self {
        //load .env into environment variables
        dotenvy::dotenv().ok();

        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .expect("Failed to build config");

        cfg.try_deserialize()
            .expect("Failed to deserialize config")
    }
}