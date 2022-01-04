use crate::error::ErrorType;
use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub struct AppError {
    pub err_type: ErrorType,
    pub message: String,
}

impl AppError {
    pub fn new(message: &str, err_type: ErrorType) -> AppError {
        AppError {
            message: message.to_string(),
            err_type,
        }
    }

    pub fn to_http_status(&self) -> warp::http::StatusCode {
        match self.err_type {
            ErrorType::NotFound => warp::http::StatusCode::NOT_FOUND,
            ErrorType::Internal => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadRequest => warp::http::StatusCode::BAD_REQUEST,
        }
    }

    pub fn from_diesel_err(err: diesel::result::Error, context: &str) -> AppError {
        AppError::new(
            format!("{}: {}", context, err).as_str(),
            match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => ErrorType::BadRequest,
                diesel::result::Error::NotFound => ErrorType::NotFound,
                // Here we can handle other cases if needed
                _ => ErrorType::Internal,
            },
        )
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for AppError {}
