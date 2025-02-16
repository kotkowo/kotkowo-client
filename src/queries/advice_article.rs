#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::queries::{
    announcement::AnnouncementTagRelationResponseCollection,
    announcement_article::ArticleEntityResponse,
};
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct GetAdviceArticleVariables {
    pub id: cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetAdviceArticleVariables")]
pub struct GetAdviceArticle {
    #[arguments(id: $id)]
    pub advice: Option<AdviceEntityResponse>,
}
#[derive(cynic::QueryFragment, Debug)]
pub struct AdviceEntityResponse {
    pub data: Option<AdviceEntity>,
}
#[derive(cynic::QueryFragment, Debug)]
pub struct AdviceEntity {
    pub attributes: Option<Advice>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Advice {
    pub article: Option<ArticleEntityResponse>,
    pub title: String,
    pub tags: Option<AnnouncementTagRelationResponseCollection>,
}
