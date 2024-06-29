#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::commons::*;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct GetArticleVariables {
    pub id: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetArticleVariables")]
pub struct GetArticle {
    #[arguments(id: $id)]
    pub announcement: Option<AnnouncementEntityResponse>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnnouncementEntityResponse {
    pub data: Option<AnnouncementEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnnouncementEntity {
    pub attributes: Option<Announcement>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Announcement {
    pub article: Option<ArticleEntityResponse>,
    pub title: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ArticleEntityResponse {
    pub data: Option<ArticleEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ArticleEntity {
    pub attributes: Option<Article>,
    pub id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Article {
    pub image: UploadFileEntityResponse,
    pub content: String,
    #[cynic(rename = "Introduction")]
    pub introduction: String,
}
