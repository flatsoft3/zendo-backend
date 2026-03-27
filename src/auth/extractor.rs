use crate::{
    auth::jwt::{Claims, JwtUtil},
    config::AppConfig,
    error::AppError,
    state::AppState,
}; 
use axum::extract::FromRequestParts;
use jsonwebtoken::dangerous::insecure_decode;
use axum::{ http::request::Parts};

pub struct AuthUser {
    pub user_id: String,
    pub full_name: String,
    pub user_type: String,
}

// ✅ no #[async_trait] needed in axum 0.8!
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let auth_header = parts
                .headers
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .ok_or(AppError::bad_request("Missing token"))?;

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or(AppError::bad_request("Invalid token format"))?;

            let unverified = insecure_decode::<Claims>(token)
                .map_err(|_| AppError::unauthorized("Unauthorized to access resource"))?;

            let user_type = unverified.claims.user_type;
            let jwt_key = get_key(&state.config, &user_type);

            let claims = JwtUtil::verify_token(jwt_key, token)
                .map_err(|_| AppError::bad_request("Invalid token"))?;

            Ok(AuthUser {
                user_id: claims.sub,
                full_name: claims.full_name,
                user_type: claims.user_type,
            })
        }
    }
}

fn get_key<'a>(config: &'a AppConfig, user_type: &str) -> &'a str {
    match user_type {
        "basic_user" => config.jwt_user_key.as_str(),
        _ => config.jwt_user_key.as_str(),
    }
}
