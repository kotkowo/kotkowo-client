mod models;
mod queries;
mod schema;

pub use models::{Age, Cat, Color, Paged, Sex};
pub use queries::cat::PaginationArg;

use std::env::VarError;

use cynic::http::CynicReqwestError;
use queries::cat::{BooleanFilterInput, CatFiltersInput, CatTagFiltersInput, StringFilterInput};
use reqwest::header::InvalidHeaderValue;
use snafu::{OptionExt, ResultExt};

use snafu::{Backtrace, Snafu};

// this should work fine but breaks rust-analyzer
// pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn get_cat(id: String) -> Result<Cat, Error> {
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

    let cat_entity = response
        .data
        .context(MissingAttributeSnafu {})?
        .cat
        .context(MissingAttributeSnafu {})?
        .data
        .context(MissingAttributeSnafu {})?;

    let source_cat = cat_entity.attributes.context(MissingAttributeSnafu {})?;

    let cat: Cat = Cat {
        id: cat_entity.id.map(|id| id.into_inner()),
        ..source_cat.into()
    };

    Ok(cat)
}

pub fn list_cat(options: Options<CatFilter>) -> Result<Paged<Cat>, Error> {
    use crate::queries::cat::ListCatVariables;
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::cat::ListCat;

    let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";

    let filters: CatFiltersInput = options
        .filter
        .map_or_else(CatFiltersInput::default, |filter| filter.into());
    let pagination = options.pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };
    let vars = ListCatVariables {
        filters,
        pagination,
        sort,
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

    let source_cats = response
        .data
        .context(MissingAttributeSnafu {})?
        .cats
        .context(MissingAttributeSnafu {})?;

    let meta = source_cats.meta;

    let cats: Result<Vec<Cat>, Error> = source_cats
        .data
        .into_iter()
        .map(|cat_entity| {
            let id = cat_entity.id.map(|id| id.into_inner());
            cat_entity
                .attributes
                .context(MissingAttributeSnafu {})
                .map(|cat| Cat { id, ..cat.into() })
        })
        .collect();

    let page: Paged<Cat> = Paged::new(meta.pagination, cats?);

    Ok(page)
}

fn get_client() -> Result<reqwest::blocking::Client, Error> {
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

#[cfg(feature = "elixir_support")]
impl rustler::Encoder for Error {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        (1, 2).encode(env)
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Opts"
)]
pub struct Options<
    #[cfg(not(feature = "elixir_support"))] F,
    #[cfg(feature = "elixir_support")] F: rustler::Encoder + for<'a> rustler::Decoder<'a>,
> {
    pub filter: Option<F>,
    pub pagination: Option<PaginationArg>,
    pub sort: Vec<String>,
}

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Cat.Filter"
)]
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
        let tags_filters: Option<Vec<Option<CatTagFiltersInput>>> = val.tags.map(|tags| {
            tags.into_iter()
                .map(|tag| {
                    Some(CatTagFiltersInput {
                        text: Some(StringFilterInput {
                            containsi: Some(tag),
                            ..StringFilterInput::default()
                        }),
                        ..CatTagFiltersInput::default()
                    })
                })
                .collect()
        });

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
                or: tags_filters,
                ..CatTagFiltersInput::default()
            }),
            ..CatFiltersInput::default()
        }
    }
}
