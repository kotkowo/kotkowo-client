#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::commons::*;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct ListAnnouncementsVariables<'a> {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnnouncementFiltersInput<'a>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationArg>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListAnnouncementsVariables")]
pub struct ListAnnouncements {
    #[arguments(filters: $filters, pagination: $pagination, sort: $sort)]
    pub announcements: Option<AnnouncementEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnnouncementEntityResponseCollection {
    pub data: Vec<AnnouncementEntity>,
    pub meta: ResponseCollectionMeta,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnnouncementEntity {
    pub attributes: Option<Announcement>,
    pub id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Announcement {
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub title: String,
    pub image: UploadFileEntityResponse,
    pub published_at: Option<DateTime>,
}

#[derive(cynic::InputObject, Debug)]
pub struct AnnouncementFiltersInput<'a> {
    pub id: Option<IdfilterInput<'a>>,
    pub title: Option<StringFilterInput>,
    #[cynic(rename = "announcement_tags")]
    pub announcement_tags: Option<AnnouncementTagFiltersInput<'a>>,
    pub article: Option<ArticleFiltersInput<'a>>,
    pub created_at: Option<DateTimeFilterInput>,
    pub updated_at: Option<DateTimeFilterInput>,
    pub published_at: Option<DateTimeFilterInput>,
    pub and: Option<Vec<Option<AnnouncementFiltersInput<'a>>>>,
    pub or: Option<Vec<Option<AnnouncementFiltersInput<'a>>>>,
    pub not: Option<Box<AnnouncementFiltersInput<'a>>>,
}

#[derive(cynic::InputObject, Debug)]
pub struct ArticleFiltersInput<'a> {
    pub id: Option<IdfilterInput<'a>>,
    #[cynic(rename = "Introduction")]
    pub introduction: Option<StringFilterInput>,
    pub announcement: Option<Box<AnnouncementFiltersInput<'a>>>,
    pub content: Option<StringFilterInput>,
    pub created_at: Option<DateTimeFilterInput>,
    pub updated_at: Option<DateTimeFilterInput>,
    pub published_at: Option<DateTimeFilterInput>,
    pub and: Option<Vec<Option<ArticleFiltersInput<'a>>>>,
    pub or: Option<Vec<Option<ArticleFiltersInput<'a>>>>,
    pub not: Option<Box<ArticleFiltersInput<'a>>>,
}

#[derive(cynic::InputObject, Debug)]
pub struct AnnouncementTagFiltersInput<'a> {
    pub id: Option<IdfilterInput<'a>>,
    pub text: Option<StringFilterInput>,
    pub created_at: Option<DateTimeFilterInput>,
    pub updated_at: Option<DateTimeFilterInput>,
    pub and: Option<Vec<Option<AnnouncementTagFiltersInput<'a>>>>,
    pub or: Option<Vec<Option<AnnouncementTagFiltersInput<'a>>>>,
    pub not: Option<Box<AnnouncementTagFiltersInput<'a>>>,
}
