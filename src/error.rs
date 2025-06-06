use thiserror::Error;

use crate::algorithm::Algorithm;

#[derive(Error, Debug, PartialEq)]
pub enum InvalidError {
    #[error("unable to decode base64 data in token")]
    Base64(#[from] base64::DecodeError),
    #[error("invalid json in token")]
    Json(String),
    #[error("opaque crypto error")]
    Crypto,
    #[error("improperly formatted token")]
    TokenFormat(String),
    #[error("invalid token claims")]
    InvalidClaims(String),
    #[error("invalid JWT key id")]
    InvalidKeyId,
}

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("invalid JWT token")]
    InvalidToken(#[from] InvalidError),
    #[error("unable to fetch token signing keys")]
    RetrieveKeyFailure,
    #[error("verification algorithm is {0:?} (not RS256)")]
    UnsupportedAlgorithm(Algorithm),
    #[error("token expired")]
    Expired,
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::InvalidToken(InvalidError::Base64(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::InvalidToken(InvalidError::Json(e.to_string()))
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(_: ring::error::Unspecified) -> Self {
        // https://docs.rs/ring/0.17.8/ring/error/struct.Unspecified.html
        Error::InvalidToken(InvalidError::Crypto)
    }
}
