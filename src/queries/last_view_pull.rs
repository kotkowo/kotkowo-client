#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::commons::*;
use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct UpdateLastPullVariables {
    pub data: LastViewPullInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct GetLastViewPull {
    pub last_view_pull: Option<LastViewPullEntityResponse>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UpdateLastPullVariables")]
pub struct UpdateLastPull {
    #[allow(unused)]
    #[arguments(data: $data)]
    pub update_last_view_pull: Option<LastViewPullEntityResponse>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LastViewPullEntityResponse {
    pub data: Option<LastViewPullEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LastViewPullEntity {
    pub attributes: Option<LastViewPull>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct LastViewPull {
    #[cynic(rename = "pull_date")]
    pub pull_date: DateTime,
}

#[derive(cynic::InputObject, Debug)]
pub struct LastViewPullInput {
    #[cynic(rename = "pull_date")]
    pub pull_date: Option<DateTime>,
}
