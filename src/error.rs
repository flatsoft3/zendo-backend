use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
// use thiserror::Error;

use crate::common::structs::ApiResponse;

// #[derive(Debug, Error)]
// pub enum AppError{
//     #[error("resource not found")]
//     NotFound,

//     #[error("bad request: {0}")]
//     BadRequest(String),

//     #[error("database error")]
//     Database(#[from] sqlx::Error),

//     #[error("internal server error")]
//     Internal,
// }

#[derive(Debug)] 
pub struct AppError {
    status: StatusCode,
    message: String,
}

//helper methods for app error
impl AppError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: message.into(),
        }
    }
}

//required by axum to wire app error into http response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body: ApiResponse<()>  = ApiResponse::error(self.status.as_u16(), self.message);

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
