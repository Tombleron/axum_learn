use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInExtension,

    TicketDeleteIdNotFound { id: u64 },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // -- Login
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_ERROR),

            // -- Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInExtension => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Model
            Self::TicketDeleteIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_ERROR,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
