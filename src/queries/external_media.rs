#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::queries::announcement::{
    AnnouncementTagFiltersInput, AnnouncementTagRelationResponseCollection,
};
pub use crate::queries::commons::*;
use crate::schema;

#[derive(cynic::InputObject, Debug)]
pub struct ExternalMediaFiltersInput<'a> {
    pub id: Option<IdfilterInput<'a>>,
    pub title: Option<StringFilterInput>,
    pub tags: Option<AnnouncementTagFiltersInput<'a>>,
    #[cynic(rename = "media_url")]
    pub media_url: Option<StringFilterInput>,
    pub created_at: Option<DateTimeFilterInput>,
    pub updated_at: Option<DateTimeFilterInput>,
    pub published_at: Option<DateTimeFilterInput>,
    pub and: Option<Vec<Option<ExternalMediaFiltersInput<'a>>>>,
    pub or: Option<Vec<Option<ExternalMediaFiltersInput<'a>>>>,
    pub not: Option<Box<ExternalMediaFiltersInput<'a>>>,
}
#[derive(cynic::QueryVariables, Debug)]
pub struct ListExternalMediaVariables<'a> {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ExternalMediaFiltersInput<'a>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationArg>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListExternalMediaVariables")]
pub struct ListExternalMedia {
    #[arguments(filters: $filters, pagination: $pagination, sort: $sort)]
    pub external_medias: Option<ExternalMediaEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ExternalMediaEntityResponseCollection {
    pub meta: ResponseCollectionMeta,
    pub data: Vec<ExternalMediaEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ExternalMediaEntity {
    pub attributes: Option<ExternalMedia>,
    pub id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ExternalMedia {
    pub image: UploadFileEntityResponse,
    pub tags: Option<AnnouncementTagRelationResponseCollection>,
    pub title: String,
    #[cynic(rename = "media_url")]
    pub media_url: String,
}
