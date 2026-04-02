use serde::{Deserialize};
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5))]
    pub full_name: String,
    #[validate(length(min =8, message = "Password must be at least 8 characters"))]
    pub password: String
}

#[derive(serde::Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min =8,  message = "Password must be at least 8 characters"))]
    pub password: String,
}