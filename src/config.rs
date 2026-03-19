use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig{
    pub app_name: String,
    pub app_env: String,
    pub app_port: u16,
    pub database_url: String,
    pub app_url: String,
    pub jwt_user_key: String,
    pub jwt_expiry: u16,
}

impl AppConfig {
    pub fn load() -> Self {
        //load .env into environment variables
        dotenvy::dotenv().ok();

        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("Failed to build config");

        cfg.try_deserialize()
            .expect("Failed to deserialize config")
    }
}