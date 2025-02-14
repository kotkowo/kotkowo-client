#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::cat_commons::*;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct ListLookingForAdoptionVariables<'a> {
    pub filters: CatFiltersInput<'a>,
    pub pagination: PaginationArg,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub owned_by_kotkowo: Option<bool>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListLookingForAdoptionVariables")]
pub struct ListLookingForAdoptionQuery {
    #[arguments(filters: { cat: $filters, owned_by_kotkowo: { eq: $owned_by_kotkowo } }, pagination: $pagination, sort: $sort)]
    pub looking_for_adoption_cats: Option<LookingForAdoptionCatEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LookingForAdoptionCatEntityResponseCollection {
    pub data: Vec<LookingForAdoptionCatEntity>,
    pub meta: ResponseCollectionMeta,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LookingForAdoptionCatEntity {
    pub id: Option<cynic::Id>,
    pub attributes: Option<LookingForAdoptionCat>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LookingForAdoptionCat {
    pub cat: Option<CatEntityResponse>,
    pub caretaker: Option<ContactInformationEntityResponse>,
}
