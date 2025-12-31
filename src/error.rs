use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Request not found: {0}")]
    NotFound(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, data) = match &self {
            AppError::DatabaseError(e) => {
                eprintln!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        status: 99,
                        message: "Database error".to_owned(),
                    }
                )
            },
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: 404,
                    message: format!("Resource not found: {}", msg)
                })
            }
        };

        let body = Json(data);
        (status, body).into_response()
    }
}