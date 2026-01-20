use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Request not found: {0}")]
    NotFound(String),

    #[error("Invalid login")]
    InvalidLogin(String),

    #[error("Invalid login token")]
    InvalidToken(),

    #[error("Internal server error")]
    InternalServerError()
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
                (
                    StatusCode::NOT_FOUND,
                    ErrorResponse {
                        status: 404,
                        message: format!("Resource not found: {}", msg)
                    }
                )
            },
            AppError::InvalidLogin(_msg) => {
                (
                    StatusCode::UNAUTHORIZED,
                    ErrorResponse {
                        status: 41,
                        message: "Invalid login".to_owned(),
                    }
                )
            },
            AppError::InvalidToken() => {
                (
                    StatusCode::UNAUTHORIZED,
                    ErrorResponse {
                        status: 42,
                        message: "Invalid token".to_owned(),
                    }
                )
            },
            AppError::InternalServerError() => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        status: 50,
                        message: "Internal server error".to_owned(),
                    }
                )
            },
            AppError::Conflict(msg) => {
                (
                    StatusCode::CONFLICT,
                    ErrorResponse {
                        status: 40,
                        message: msg.clone(),
                    },
                )
            }
        };

        let body = Json(data);
        (status, body).into_response()
    }
}