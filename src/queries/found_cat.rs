#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::schema;

pub use crate::queries::cat_commons::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct ListFoundCatVariables<'a> {
    pub filters: CatFiltersInput<'a>,

    pub pagination: PaginationArg,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListFoundCatVariables")]
pub struct ListFoundCat {
    #[arguments(filters: { cat: $filters })]
    pub found_cats: Option<FoundCatEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct FoundCatEntityResponseCollection {
    pub data: Vec<FoundCatEntity>,
    pub meta: ResponseCollectionMeta,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct FoundCatEntity {
    pub id: Option<cynic::Id>,
    pub attributes: Option<FoundCat>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct FoundCat {
    #[cynic(rename = "discovery_circumstances")]
    pub discovery_circumstances: String,
    #[cynic(rename = "found_datetime")]
    pub found_datetime: DateTime,
    #[cynic(rename = "found_location")]
    pub found_location: String,
    #[cynic(rename = "special_signs")]
    pub special_signs: Option<String>,
    pub cat: Option<CatEntityResponse>,
}
