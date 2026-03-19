use serde::{Deserialize};


#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub full_name: String,
    pub password: String
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}