use http::status::StatusCode;
use oauth2::basic::BasicErrorResponseType;
use oauth2::RequestTokenError;
use reqwest::Error as ReqwestError;
// use serde_json::error::Error as SerdeJsonError;
use std::convert::From;
use thiserror::Error as ThisError;
use url::ParseError;

pub trait WithCode {
    fn code(&self) -> StatusCode;
}

#[derive(Debug, ThisError)]
pub enum AuthError {
    #[error("Malformed Redirect Url")]
    BadRedirectUrl(ParseError),
    #[error("Can't parse token `{}`", _0)]
    // ParseTokenError(SerdeJsonError),
    // #[error("Can't exchange token `{}`", _0)]
    ExchangeTokenError(RequestTokenError<BasicErrorResponseType>),
    // #[error("Failed to request token `{}`", _0)]
    // OAuth2ErrorResponse(ErrorResponse<BasicErrorResponseType>),
    #[error("Can't make HTTP Request `{}`", _0)]
    ReqwestError(ReqwestError),

    #[error("Something went wrong `{}`", _0)]
    Other(String),
}

impl WithCode for AuthError {
    fn code(&self) -> StatusCode {
        use AuthError::*;

        match self {
            BadRedirectUrl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // ParseTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ExchangeTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // OAuth2ErrorResponse(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ReqwestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ParseError> for AuthError {
    fn from(error: ParseError) -> Self {
        AuthError::BadRedirectUrl(error)
    }
}

impl From<RequestTokenError<BasicErrorResponseType>> for AuthError {
    fn from(error: RequestTokenError<BasicErrorResponseType>) -> Self {
        AuthError::ExchangeTokenError(error)
    }
}

impl From<ReqwestError> for AuthError {
    fn from(error: ReqwestError) -> Self {
        AuthError::ReqwestError(error)
    }
}
