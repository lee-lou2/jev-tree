use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("too many failed logins; try again in {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },
    #[error("store lock poisoned")]
    Poisoned,
    #[error("request cancelled")]
    Cancelled,
    #[error("request timed out")]
    Timeout,
    #[error("TypeSafe call failed: {0}")]
    Upstream(#[from] crate::jev::JevError),
    #[error("store error: {0}")]
    Storage(#[from] rusqlite::Error),
    #[error("file error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl Error {
    pub fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "invalid_request",
            Self::Conflict(_) => "conflict",
            Self::NotFound(_) => "not_found",
            Self::Unauthorized(_) => "unauthorized",
            Self::Forbidden(_) => "forbidden",
            Self::RateLimited { .. } => "rate_limited",
            Self::Upstream(crate::jev::JevError::Unconfigured) => "invalid_request",
            Self::Upstream(_) => "upstream_error",
            Self::Cancelled => "cancelled",
            Self::Timeout => "timeout",
            _ => "internal_error",
        }
    }
    pub fn public_message(&self) -> String {
        match self {
            Self::Storage(_) | Self::Io(_) | Self::Json(_) => "storage request failed".into(),
            Self::Upstream(crate::jev::JevError::Unconfigured) => {
                "Jev API key is required. Add it in Settings → Models.".into()
            }
            Self::Upstream(e) => match e {
                crate::jev::JevError::Status(401) => {
                    "Jev key rejected (401). Check the Jev API key in Settings → Models.".into()
                }
                crate::jev::JevError::Status(404) => {
                    "Jev endpoint not found (404). Item ranking posts to TypeSafe (https://api.typesafe.ai). The LLM base URL in Settings → Models is a different host.".into()
                }
                crate::jev::JevError::Status(s @ (400 | 413 | 422)) => {
                    format!(
                        "Jev rejected the request (HTTP {s}). The input may be too large for the evaluator: shorten the question, answer, or context."
                    )
                }
                crate::jev::JevError::Status(s) => {
                    format!("Jev call failed (HTTP {s}). Check the Jev API key in Settings → Models.")
                }
                _ => "no upstream response; this request did not complete".into(),
            }
            _ => self.to_string(),
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::Upstream(crate::jev::JevError::Unconfigured) => StatusCode::BAD_REQUEST,
            Self::Upstream(_) => StatusCode::BAD_GATEWAY,
            Self::Cancelled => StatusCode::from_u16(499).unwrap_or(StatusCode::BAD_REQUEST),
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        tracing::warn!(code = self.code(), error = %self, "request failed");
        let retry_after = match &self {
            Self::RateLimited { retry_after_secs } => Some(*retry_after_secs),
            _ => None,
        };
        let mut response = (
            status,
            Json(json!({"code":self.code(),"detail":self.public_message()})),
        )
            .into_response();
        if let Some(seconds) = retry_after {
            response
                .headers_mut()
                .insert(axum::http::header::RETRY_AFTER, seconds.into());
        }
        response
    }
}
