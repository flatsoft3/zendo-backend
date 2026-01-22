use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig{
    pub app_name: String,
    pub app_env: String,
    pub app_port: u16,
    pub database_url: String
}

impl AppConfig {
    pub fn load() -> Self {
        //load .env into environment variables
        dotenvy::dotenv().ok();

        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("Failed to buil config");

        cfg.try_deserialize()
            .expect("Failed to deserialize config")
    }
}