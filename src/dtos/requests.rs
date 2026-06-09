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

#[derive(Deserialize, Validate)]
pub struct UpdateCompanyRequest {
    #[validate(length(min = 5, message = "Company name must be at least 5 characters"))]
    pub company_name: String,

    #[validate(length(min = 3, message = "RC number must be at least 3 characters"))]
    pub rc_number: String,

    #[validate(length(min = 8, message = "Tax ID must be at least 8 characters"))]
    pub tax_id: Option<String>,

    #[validate(length(min = 5, message = "Company address must be at least 5 characters"))]
    pub company_address: String,
}