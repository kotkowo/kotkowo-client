#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::commons::*;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct ArticleViewCountQueryVariables<'a> {
    pub article_id: Option<&'a cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ArticleViewCountQueryVariables")]
pub struct ArticleViewCountQuery {
    #[arguments(filters: { article: { id: { eq: $article_id } } })]
    pub article_views: Option<ArticleViewEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ArticleViewEntityResponseCollection {
    pub meta: ResponseCollectionMeta,
}
