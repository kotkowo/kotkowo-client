#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::schema;

pub use crate::queries::cat_commons::*;
#[derive(cynic::QueryVariables, Debug)]
pub struct ListSupporterWithCatsVariables {
    pub pagination: PaginationArg,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListSupporterWithCatsVariables")]
pub struct ListSupporterWithCats {
    #[arguments(pagination: $pagination, sort: $sort)]
    pub supporters: Option<SupporterEntityResponseCollection>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ListVirtualCatVariables<'a> {
    pub filters: CatFiltersInput<'a>,
    pub pagination: PaginationArg,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListVirtualCatVariables")]
pub struct ListVirtualCat {
    #[arguments(filters: { cat: $filters })]
    pub virtual_cats: Option<VirtualCatEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct VirtualCatEntityResponseCollection {
    pub data: Vec<VirtualCatEntity>,
    pub meta: ResponseCollectionMeta,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct VirtualCatEntity {
    pub id: Option<cynic::Id>,
    pub attributes: Option<VirtualCat>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct VirtualCat {
    pub cat: Option<CatEntityResponse>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct SupporterEntityResponseCollection {
    pub meta: ResponseCollectionMeta,
    pub data: Vec<SupporterEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct SupporterEntity {
    pub attributes: Option<Supporter>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Supporter {
    #[cynic(rename = "contact_information")]
    pub contact_information: Option<ContactInformationEntityResponse>,
    pub portrait: Option<ImageEntityResponse>,
    #[cynic(rename = "virtual_cats")]
    pub virtual_cats: Option<VirtualCatRelationResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct VirtualCatRelationResponseCollection {
    pub data: Vec<VirtualCatEntity>,
}
