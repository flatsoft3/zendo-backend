use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: Option<usize>,
    pub iss: String,
    pub iat: DateTime<Utc>,

    pub full_name: String,
    pub user_type: String,
}

pub struct JwtUtil {}

impl JwtUtil {
    pub fn generate_token(
        iss: &str,
        exp: Option<usize>,
        key: &str,
        user_id: &str,
        full_name: &str,
        user_type: &str,
    ) -> Result<String, AppError> {
        let expiration = exp.map(|seconds| {
            Utc::now()
                .checked_add_signed(Duration::seconds(seconds.try_into().unwrap()))
                .unwrap()
                .timestamp() as usize
        });

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iss: iss.to_string(),
            iat: Utc::now(),
            full_name: full_name.to_string(),
            user_type: user_type.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(key.as_bytes()),
        )
        .map_err(AppError::from)
    }

    pub fn verify_token(key: &str, token: &str) -> Result<Claims, AppError> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::default(),
        )?;

        Ok(data.claims)
    }
}
