#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::queries::announcement::AnnouncementTagRelationResponseCollection;
pub use crate::queries::commons::*;
use crate::schema;

use super::announcement::{AnnouncementTagFiltersInput, ArticleFiltersInput};

#[derive(cynic::QueryVariables, Debug)]
pub struct ListAdviceVariables<'a> {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AdviceFiltersInput<'a>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationArg>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListAdviceVariables")]
pub struct ListAdvice {
    #[arguments(pagination: $pagination, filters: $filters, sort: $sort)]
    pub advices: Option<AdviceEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AdviceEntityResponseCollection {
    pub data: Vec<AdviceEntity>,
    pub meta: ResponseCollectionMeta,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AdviceEntity {
    pub attributes: Option<Advice>,
    pub id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Advice {
    pub title: String,
    pub image: UploadFileEntityResponse,
    pub tags: Option<AnnouncementTagRelationResponseCollection>,
}

#[derive(cynic::InputObject, Debug)]
pub struct AdviceFiltersInput<'a> {
    pub id: Option<IdfilterInput<'a>>,
    pub title: Option<StringFilterInput>,
    pub tags: Option<AnnouncementTagFiltersInput<'a>>,
    pub article: Option<ArticleFiltersInput<'a>>,
    pub created_at: Option<DateTimeFilterInput>,
    pub updated_at: Option<DateTimeFilterInput>,
    pub published_at: Option<DateTimeFilterInput>,
    pub and: Option<Vec<Option<AdviceFiltersInput<'a>>>>,
    pub or: Option<Vec<Option<AdviceFiltersInput<'a>>>>,
    pub not: Option<Box<AdviceFiltersInput<'a>>>,
}
