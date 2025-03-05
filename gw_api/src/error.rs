use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = std::result::Result<T, GatewayError>;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    Validation,
}

#[derive(Debug)]
pub struct GatewayError {
    pub kind: ErrorKind,
    pub message: String,
}

impl IntoResponse for GatewayError {
    fn into_response(self) -> axum::response::Response {
        let code = match self.kind {
            ErrorKind::Validation => StatusCode::BAD_REQUEST,
        };
        let error = match self.kind {
            ErrorKind::Validation => "VALIDATION",
        };
        let obj = json!({
            "error": error,
            "message": self.message
        });
        (code, Json(obj)).into_response()
    }
}

impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.kind {
            ErrorKind::Validation => "ValidationError",
        };
        write!(f, "{prefix}: {}", self.message)
    }
}

impl std::error::Error for GatewayError {}
