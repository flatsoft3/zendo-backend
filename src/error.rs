use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError{
    #[error("resource not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("internal server error")]
    Internal,
}
impl IntoResponse for AppError {
    fn into_response (self) -> Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

// impl From<sqlx::Error> for AppError {
//     fn from(error: sqlx::Error) -> Self {
//         match error {
//             sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".into()),
//             _ => AppError::DatabaseError(error.to_string()),
//         }
//     }
// }
