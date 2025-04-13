#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct UpdateViewsVariables {
    pub pull_date: String,
    pub updates: Vec<IncrementFieldInput>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UpdateViewsVariables")]
pub struct UpdateViews {
    #[arguments(pull_date: $pull_date, updates: $updates)]
    pub increment_fields_and_update_pull_date: Option<bool>,
}

#[derive(cynic::InputObject, Debug)]
pub struct IncrementFieldInput {
    pub content_type: String,
    pub id: cynic::Id,
    pub field: String,
    pub amount: i32,
}
