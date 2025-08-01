use actix_web::{http::StatusCode, ResponseError};
use webauthn_rs::prelude::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User not found")]
    UserNotFound,

    #[error("Unknown webauthn error")]
    Unknown(WebauthnError),

    #[error("Bad request")]
    BadRequest(#[from] WebauthnError),

    #[error("User has no credentials")]
    UserHasNoCredentials,

    #[error("Registration state is corrupted: {0}")]
    CorruptRegistrationState(String),

    #[error("Authentication state is corrupted: {0}")]
    CorruptAuthenticationState(String),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::UserNotFound | Error::BadRequest(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
