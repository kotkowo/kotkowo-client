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
            Error::MissingAttribute { backtrace } => {
                format!("MissingAttribute:\n Backtrace:\n {}", backtrace)
            }
            Error::CynicRequestError { source, backtrace } => {
                format!("CynicRequestError\nSource: {source}\nBacktrace: {backtrace}")
            }
            Error::RequestError { source, backtrace } => {
                format!("ReqwestError\n Source: {source}\n Backtrace: {backtrace}")
            }
            Error::RequestResultedInError { message } => message.to_owned(),
            Error::EnvVarMissing { source, backtrace } => {
                format!("EnvVarMissing\n Source: {source}\n Backtrace: {backtrace}")
            }
            Error::InvalidHeaderValue { source, backtrace } => {
                format!("InvalidHeaderValue\n Source: {source}\n Backtrace: {backtrace}")
            }
        };
        // let msg = &self.to_string();

        msg.encode(env)
    }
}
