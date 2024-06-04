#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::commons::*;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct ArticleViewCountQueryVariables<'a> {
    pub article_id: Option<IdfilterInput<'a>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ArticleViewCountQueryVariables")]
pub struct ArticleViewCountQuery {
    pub article_views: Option<ArticleViewEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ArticleViewEntityResponseCollection {
    pub meta: ResponseCollectionMeta,
}
