#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::schema;

pub use crate::queries::cat_commons::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct ListLostCatVariables<'a> {
    pub filters: CatFiltersInput<'a>,
    pub pagination: PaginationArg,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListLostCatVariables")]
pub struct ListLostCat {
    #[arguments(filters: { cat: $filters })]
    pub lost_cats: Option<LostCatEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LostCatEntityResponseCollection {
    pub meta: ResponseCollectionMeta,
    pub data: Vec<LostCatEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LostCatEntity {
    pub attributes: Option<LostCat>,
    pub id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LostCat {
    #[cynic(rename = "disappearance_circumstances")]
    pub disappearance_circumstances: String,
    #[cynic(rename = "disappearance_datetime")]
    pub disappearance_datetime: DateTime,
    #[cynic(rename = "disappearance_location")]
    pub disappearance_location: String,
    #[cynic(rename = "during_medical_treatment")]
    pub during_medical_treatment: bool,
    #[cynic(rename = "special_signs")]
    pub special_signs: Option<String>,
    pub cat: Option<CatEntityResponse>,
}
