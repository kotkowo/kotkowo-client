#![feature(assert_matches)]

mod queries;
mod schema;

use std::env::VarError;

use cynic::http::CynicReqwestError;
use queries::cat::{BooleanFilterInput, CatFiltersInput, CatTagFiltersInput, StringFilterInput};
use reqwest::header::InvalidHeaderValue;
use snafu::{OptionExt, ResultExt};

use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
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

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn get_cat(id: String) -> Result<queries::cat::Cat> {
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::cat::{GetCat, GetCatVariables};

    let id: cynic::Id = id.into();
    let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";
    let operation = GetCat::build(GetCatVariables { id: &id });
    let client = get_client()?;
    let response = client
        .post(endpoint)
        .run_graphql(operation)
        .context(CynicRequestSnafu {})?;

    if let Some(err) = response.errors {
        let message = format!("{:?}", err).to_string();
        return Err(Error::RequestResultedInError { message });
    }

    let cat = response
        .data
        .context(MissingAttributeSnafu {})?
        .cat
        .context(MissingAttributeSnafu {})?
        .data
        .context(MissingAttributeSnafu {})?
        .attributes
        .context(MissingAttributeSnafu {})?;

    Ok(cat)
}

#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug)]
pub enum Age {
    Senior,
    Adult,
    Junior,
}

#[derive(Debug)]
pub enum Color {
    Black,
    Gray,
    Tricolor,
    Patched,
    Ginger,
    OtherColor,
}

#[derive(Debug, Default)]
pub struct CatFilter {
    sex: Option<Sex>,
    age: Option<Age>,
    color: Option<Color>,
    castrated: Option<bool>,
    tags: Option<Vec<String>>,
}

impl<'a> From<CatFilter> for CatFiltersInput<'a> {
    fn from(val: CatFilter) -> Self {
        let sex_eq: Option<String> = val.sex.map(|sex| format!("{:?}", sex));
        let age_eq: Option<String> = val.age.map(|age| format!("{:?}", age));
        let color_eq: Option<String> = val.color.map(|color| format!("{:?}", color));
        let tags_in: Option<Vec<Option<String>>> =
            val.tags.map(|tags| tags.into_iter().map(Some).collect());

        CatFiltersInput {
            castrated: Some(BooleanFilterInput {
                eq: val.castrated,
                ..BooleanFilterInput::default()
            }),
            color: Some(StringFilterInput {
                eq: color_eq,
                ..StringFilterInput::default()
            }),
            age: Some(StringFilterInput {
                eq: age_eq,
                ..StringFilterInput::default()
            }),
            sex: Some(StringFilterInput {
                eqi: sex_eq,
                ..StringFilterInput::default()
            }),
            cat_tags: Some(CatTagFiltersInput {
                text: Some(StringFilterInput {
                    in_: tags_in,
                    ..StringFilterInput::default()
                }),
                ..CatTagFiltersInput::default()
            }),
            ..CatFiltersInput::default()
        }
    }
}

pub fn list_cat(filters: Option<CatFilter>) -> Result<Vec<queries::cat::Cat>> {
    use crate::queries::cat::ListCatVariables;
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::cat::ListCat;

    let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";

    let vars = match filters {
        None => ListCatVariables {
            filters: CatFiltersInput::default(),
        },
        Some(filters) => ListCatVariables {
            filters: filters.into(),
        },
    };

    let operation = ListCat::build(vars);
    let client = get_client()?;
    let response = client
        .post(endpoint)
        .run_graphql(operation)
        .context(CynicRequestSnafu {})?;

    if let Some(err) = response.errors {
        let message = format!("{:?}", err).to_string();
        return Err(Error::RequestResultedInError { message });
    }

    let cat: Result<Vec<queries::cat::Cat>> = response
        .data
        .context(MissingAttributeSnafu {})?
        .cats
        .context(MissingAttributeSnafu {})?
        .data
        .into_iter()
        .map(|cat_entity| cat_entity.attributes.context(MissingAttributeSnafu {}))
        .collect();

    cat
}

fn get_client() -> Result<reqwest::blocking::Client> {
    let api_key = std::env::var("STRAPI_KEY").context(EnvVarMissingSnafu {})?;
    let mut headers = reqwest::header::HeaderMap::with_capacity(1);
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key)
            .parse()
            .context(InvalidHeaderValueSnafu {})?,
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .context(RequestSnafu {})?;

    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cat() {
        let resp = get_cat("1".into());

        insta::assert_snapshot!(format!("{:?}", resp));
    }

    #[test]
    fn test_list_cat() {
        let resp = list_cat(Some(CatFilter {
            sex: Some(Sex::Female),
            tags: Some(vec!["Test".to_string()]),
            ..CatFilter::default()
        }));

        insta::assert_snapshot!(format!("{:?}", resp));
    }
}
