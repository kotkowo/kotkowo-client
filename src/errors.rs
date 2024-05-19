use cynic::http::CynicReqwestError;
use reqwest::header::InvalidHeaderValue;
use snafu::{Backtrace, Snafu};
use std::env::VarError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Missing or none attribute"))]
    MissingAttribute { backtrace: Backtrace },

    #[snafu(display("Request failure"))]
    CynicRequestError {
        source: CynicReqwestError,
        backtrace: Backtrace,
    },

    #[snafu(display("Request failure"))]
    RequestError {
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("{:?}", message))]
    RequestResultedInError { message: String },

    #[snafu(display("Environment variable missing"))]
    EnvVarMissing {
        source: VarError,
        backtrace: Backtrace,
    },

    #[snafu(display("Invalid header value"))]
    InvalidHeaderValue {
        source: InvalidHeaderValue,
        backtrace: Backtrace,
    },
}

#[cfg(feature = "elixir_support")]
impl rustler::Encoder for Error {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        let msg: String = match self {
            Error::MissingAttribute { backtrace: _ } => "MissingAttribute".to_string(),
            Error::CynicRequestError {
                source: _,
                backtrace: _,
            } => "CynicRequestError".to_string(),
            Error::RequestError {
                source: _,
                backtrace: _,
            } => "ReqwestError".to_string(),
            Error::RequestResultedInError { message } => message.to_owned(),
            Error::EnvVarMissing {
                source: _,
                backtrace: _,
            } => "EnvVarMissing".to_string(),
            Error::InvalidHeaderValue {
                source: _,
                backtrace: _,
            } => "InvalidHeaderValue".to_string(),
        };
        // let msg = &self.to_string();

        msg.encode(env)
    }
}
