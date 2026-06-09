use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize)]
pub struct CompanyDetailsResponse {
    pub user_id: Uuid,
    pub company_name: Option<String>,
    pub rc_number: Option<String>,
    pub tax_id: Option<String>,
    pub company_address: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for CompanyDetailsResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            company_name: user.company_name,
            rc_number: user.rc_number,
            tax_id: user.tax_id,
            company_address: user.company_address,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct UserCreatedResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub phone_number: String,
    pub company_name: Option<String>,
    pub rc_number: Option<String>,
    pub tax_id: Option<String>,
    pub company_address: Option<String>,
    pub kyc_tier: i16,
    pub kyc_verified_at: Option<DateTime<Utc>>,
    pub account_status: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserCreatedResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            middle_name: user.middle_name,
            last_name: user.last_name,
            phone_number: user.phone_number,
            company_name: user.company_name,
            rc_number: user.rc_number,
            tax_id: user.tax_id,
            company_address: user.company_address,
            kyc_tier: user.kyc_tier,
            kyc_verified_at: user.kyc_verified_at,
            account_status: user.account_status,
            email_verified_at: user.email_verified_at,
            created_at: user.created_at,
        }
    }
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_expiry: u32,
    pub user_info: UserCreatedResponse,
}
