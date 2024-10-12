use core::fmt;

use axum::{http::StatusCode, response::IntoResponse};
use deadpool_diesel::InteractError;

#[derive(Debug)]
pub enum InfrastructureError {
    NotFound,
    InternalError,
}

impl IntoResponse for InfrastructureError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED EXCEPTION").into_response()
    }
}

// converts Error into InfrastructureError
pub trait Error {
    fn as_infra_error(&self) -> InfrastructureError;
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfrastructureError::InternalError => write!(f, "Internal server error"),
            InfrastructureError::NotFound => write!(f, "not found"),
        }
    }
}

// Implement the Error trait for diesel::result::Error
impl Error for diesel::result::Error {
    fn as_infra_error(&self) -> InfrastructureError {
        match self {
            diesel::result::Error::NotFound => InfrastructureError::NotFound,
            _ => InfrastructureError::InternalError,
        }
    }
}

// Implement the Error trait for deadpool_diesel::PoolError
impl Error for deadpool_diesel::PoolError {
    fn as_infra_error(&self) -> InfrastructureError {
        InfrastructureError::InternalError
    }
}

// Implement the Error trait for InteractError
impl Error for InteractError {
    fn as_infra_error(&self) -> InfrastructureError {
        InfrastructureError::InternalError
    }
}

// Utility function to adapt errors of generic type T into InfraError
pub fn adapt_infra_error<T: Error>(error: T) -> InfrastructureError {
    error.as_infra_error()
}
