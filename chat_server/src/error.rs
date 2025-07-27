use axum::{
    Json,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("sql error: {0}")]
    SqlError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("Http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            AppError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            AppError::SqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => StatusCode::FORBIDDEN,
            AppError::HttpHeaderError(_) => StatusCode::BAD_REQUEST,
        };
        (status, Json(json!({"error": self.to_string()}))).into_response()
    }
}
