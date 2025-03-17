use axum::{http::StatusCode, response::IntoResponse, Json};
use gw_core::error::{DbErrorKind, Error, ErrorKind as CoreErrorKind};
use serde_json::json;

pub type Result<T> = std::result::Result<T, GatewayError>;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    Validation,
    Resource,
    Fatal,
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
            ErrorKind::Resource => StatusCode::NOT_FOUND,
            ErrorKind::Fatal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error = match self.kind {
            ErrorKind::Validation => "VALIDATION",
            ErrorKind::Resource => "RESOURCE",
            ErrorKind::Fatal => "FATAL",
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
            ErrorKind::Resource => "ResourceError",
            ErrorKind::Fatal => "FatalError",
        };
        write!(f, "{prefix}: {}", self.message)
    }
}

impl std::error::Error for GatewayError {}

impl From<validify::ValidationErrors> for GatewayError {
    fn from(value: validify::ValidationErrors) -> Self {
        dbg!(value);
        GatewayError {
            kind: ErrorKind::Validation,
            message: String::from("TODO VAL ERROR"),
        }
    }
}

impl From<Error> for GatewayError {
    fn from(value: Error) -> Self {
        match value.kind {
            CoreErrorKind::Database(DbErrorKind::Query) => GatewayError {
                kind: ErrorKind::Resource,
                message: format!("{}", value.message),
            },
            CoreErrorKind::Database(..) => GatewayError {
                kind: ErrorKind::Fatal,
                message: format!("{}", value.message),
            },
            CoreErrorKind::Type => GatewayError {
                kind: ErrorKind::Fatal,
                message: "Unknown".into(),
            },
        }
    }
}
