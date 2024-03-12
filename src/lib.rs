#![feature(assert_matches)]

mod schema;
mod queries;


use std::env::VarError;

use cynic::http::CynicReqwestError;
use reqwest::header::InvalidHeaderValue;
use snafu::{OptionExt, ResultExt};

use snafu::{Snafu, Backtrace};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Missing or none attribute"))]
    MissingAttribute { backtrace: Backtrace },

    #[snafu(display("Request failure"))]
    CynicRequestError { source:  CynicReqwestError, backtrace: Backtrace },

    #[snafu(display("Request failure"))]
    RequestError { source: reqwest::Error, backtrace: Backtrace },

    #[snafu(display("Environment variable missing"))]
    EnvVarMissing { source: VarError, backtrace: Backtrace },

    #[snafu(display("Invalid header value"))]
    InvalidHeaderValue { source: InvalidHeaderValue, backtrace: Backtrace },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;


pub fn get_cat(id: String) -> Result<queries::get_cat::Cat> {
   use cynic::http::ReqwestBlockingExt;
   use cynic::QueryBuilder;
   use queries::get_cat::{GetCat, GetCatVariables};

   let id: cynic::Id = id.into();
   let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";
   let operation = GetCat::build(GetCatVariables{id: &id});
   let client = get_client()?;
   let response = client.post(endpoint).run_graphql(operation).context(CynicRequestSnafu {})?;
   let cat = response
       .data.context(MissingAttributeSnafu {})?
       .cat.context(MissingAttributeSnafu {})?
       .data.context(MissingAttributeSnafu {})?
       .attributes.context(MissingAttributeSnafu {})?;

   Ok(cat)
}

fn get_client() -> Result<reqwest::blocking::Client> {
    let api_key = std::env::var("STRAPI_KEY").context(EnvVarMissingSnafu {})?;
    let mut headers = reqwest::header::HeaderMap::with_capacity(1);
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key).parse().context(InvalidHeaderValueSnafu {})?,
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build().context(RequestSnafu {})?;

    Ok(client)
}


#[cfg(test)]
mod tests {


use super::*;

  #[test]
  fn test_get_cat(){
    let resp = get_cat("1".into());
    println!("{:?}", resp);
    // assert_eq!(resp.unwrap().data.unwrap().cat.unwrap().data.unwrap().attributes.unwrap().name, "test");
  }
}
