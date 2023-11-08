use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::routes;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFail,
    // -- CtxExtError
    CtxExt(routes::mw_auth::CtxExtError),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- ClientError

// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use routes::Error::*;

        match self {
            // -- Login/Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // -- Fallback.
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
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
// endregion: --- ClientError
