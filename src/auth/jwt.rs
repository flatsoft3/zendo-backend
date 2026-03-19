use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{config::AppConfig, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: Option<usize>,
    pub iss: String,
    pub iat: DateTime<Utc>,

    pub full_name: String,
    pub user_type: String,
}

pub struct JwtUtil {
   pub config: AppConfig,
   pub key: String,
   pub exp: Option<usize>,
}

impl JwtUtil {
    pub fn generate_token(
        &self,
        user_id: &str,
        full_name: &str,
        user_type: &str,
    ) -> Result<String, AppError> {
        let expiration = self.exp.map(|seconds| {
            Utc::now()
                .checked_add_signed(Duration::seconds(seconds.try_into().unwrap()))
                .unwrap()
                .timestamp() as usize
        });

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iss: self.config.app_url.clone(),
            iat: Utc::now(),
            full_name: full_name.to_string(),
            user_type: user_type.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.key.as_bytes()),
        )
        .map_err(AppError::from)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.key.as_bytes()),
            &Validation::default(),
        )?;

        Ok(data.claims)
    }
}
