#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::commons::*;
use crate::schema;

pub use crate::queries::cat_commons::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct GetCatVariables<'a> {
    pub id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ListCatVariables<'a> {
    pub filters: CatFiltersInput<'a>,
    pub pagination: PaginationArg,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListCatVariables")]
pub struct ListCat {
    #[arguments(filters: $filters, pagination: $pagination, sort: $sort)]
    pub cats: Option<CatEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetCatVariables")]
pub struct GetCat {
    #[arguments(id: $id)]
    pub cat: Option<CatEntityResponse>,
}
