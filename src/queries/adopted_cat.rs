#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::cat_commons::*;
pub use crate::queries::commons::*;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct AdoptedCatQueryVariables<'a> {
    pub cat: CatFiltersInput<'a>,
    pub pagination: PaginationArg,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub between: Option<Vec<Option<DateTime>>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "AdoptedCatQueryVariables")]
pub struct AdoptedCatQuery {
    #[arguments(filters: { adoption_date: { between: $between }, cat: $cat }, pagination: $pagination, sort: $sort)]
    pub adopted_cats: Option<AdoptedCatEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AdoptedCatEntityResponseCollection {
    pub data: Vec<AdoptedCatEntity>,
    pub meta: ResponseCollectionMeta,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AdoptedCatEntity {
    pub id: Option<cynic::Id>,
    pub attributes: Option<AdoptedCat>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AdoptedCat {
    #[cynic(rename = "adoption_date")]
    pub adoption_date: DateTime,
    pub cat: Option<CatEntityResponse>,
}
