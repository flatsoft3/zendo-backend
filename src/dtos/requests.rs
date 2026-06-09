use serde::{Deserialize};
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 2))]
    pub first_name: String,

    pub middle_name: Option<String>,

    #[validate(length(min = 2))]
    pub last_name: String,

    #[validate(length(min = 11))]
    pub phone_number: String,

    #[validate(length(
        min = 8,
        message = "Password must be at least 8 characters"
    ))]
    pub password: String,
}

#[derive(serde::Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min =8,  message = "Password must be at least 8 characters"))]
    pub password: String,
}