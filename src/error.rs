use std::collections::HashMap;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::common::structs::ApiResponse;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug)]
pub struct AppError {
    status: StatusCode,
    message: String,
    validation_errors: Option<HashMap<String, String>>,
}

//helper methods for app error
impl AppError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
            validation_errors: None,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
            validation_errors: None,
        }
    }
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
            validation_errors: None,
        }
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: message.into(),
            validation_errors: None,
        }
    }

    pub fn validation_error(errors: ValidationErrors) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: "Data validation failed".to_string(),
            validation_errors: Some(Self::format_validation_errors(&errors)),
        }
    }

    fn format_validation_errors(errors: &ValidationErrors) -> HashMap<String, String> {
        errors
            .errors()
            .iter()
            .filter_map(|(field, field_errors)| {
                if let ValidationErrorsKind::Field(errors_vec) = field_errors {
                    errors_vec.first().map(|error| {
                    let message = error
                        .message
                        .clone()
                        .unwrap_or_else(|| format!("Invalid {}", field).into());

                    (field.to_string(), message.to_string())

                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

//required by axum to wire app error into http response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body: ApiResponse<()> = if let Some(validation_errors) = self.validation_errors {
            ApiResponse::validation_error(self.status.as_u16(), self.message, validation_errors)
        } else {
            ApiResponse::error(self.status.as_u16(), self.message)
        };

        (self.status, Json(body)).into_response()
    }
}

//convert sqlx error into app error
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::internal(error.to_string())
        // match error {
        //     sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".into()),
        //     _ => AppError::DatabaseError(error.to_string()),
        // }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        AppError::internal(error.to_string())
    }
}
