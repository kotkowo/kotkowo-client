#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::schema;

#[derive(cynic::QueryFragment, Debug)]
pub struct ResponseCollectionMeta {
    pub pagination: Pagination,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Pagination {
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub page_count: i32,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ImageRelationResponseCollection {
    pub data: Vec<ImageEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ImageEntity {
    pub attributes: Option<Image>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Image {
    pub image: UploadFileEntityResponse,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct UploadFileEntityResponse {
    pub data: Option<UploadFileEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct UploadFileEntity {
    pub attributes: Option<UploadFile>,
    pub id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct UploadFile {
    pub url: String,
    pub height: Option<i32>,
    pub mime: String,
    pub name: String,
    pub preview_url: Option<String>,
    pub width: Option<i32>,
    pub alternative_text: Option<String>,
}

#[derive(cynic::InputObject, Debug, Default)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Pagination"
)]
pub struct PaginationArg {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
#[derive(cynic::InputObject, Debug)]
pub struct DateTimeFilterInput {
    pub and: Option<Vec<Option<DateTime>>>,
    pub or: Option<Vec<Option<DateTime>>>,
    pub not: Option<Box<DateTimeFilterInput>>,
    pub eq: Option<DateTime>,
    pub eqi: Option<DateTime>,
    pub ne: Option<DateTime>,
    pub nei: Option<DateTime>,
    pub starts_with: Option<DateTime>,
    pub ends_with: Option<DateTime>,
    pub contains: Option<DateTime>,
    pub not_contains: Option<DateTime>,
    pub containsi: Option<DateTime>,
    pub not_containsi: Option<DateTime>,
    pub gt: Option<DateTime>,
    pub gte: Option<DateTime>,
    pub lt: Option<DateTime>,
    pub lte: Option<DateTime>,
    pub null: Option<bool>,
    pub not_null: Option<bool>,
    #[cynic(rename = "in")]
    pub in_: Option<Vec<Option<DateTime>>>,
    pub not_in: Option<Vec<Option<DateTime>>>,
    pub between: Option<Vec<Option<DateTime>>>,
}

#[derive(cynic::InputObject, Debug, Default)]
#[cynic(graphql_type = "IDFilterInput")]
pub struct IdfilterInput<'a> {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Option<&'a cynic::Id>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Option<&'a cynic::Id>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<IdfilterInput<'a>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eq: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eqi: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub ne: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub nei: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub contains: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_contains: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub containsi: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_containsi: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gt: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gte: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lt: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lte: Option<&'a cynic::Id>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_null: Option<bool>,

    #[cynic(rename = "in")]
    pub in_: Option<Vec<Option<&'a cynic::Id>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<Vec<Option<&'a cynic::Id>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub between: Option<Vec<Option<&'a cynic::Id>>>,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct DateTime(pub String);

#[derive(cynic::InputObject, Debug)]
pub struct ImageFiltersInput<'a> {
    pub id: Option<IdfilterInput<'a>>,
    pub title: Option<StringFilterInput>,
    pub created_at: Option<DateTimeFilterInput>,
    pub updated_at: Option<DateTimeFilterInput>,
    pub and: Option<Vec<Option<ImageFiltersInput<'a>>>>,
    pub or: Option<Vec<Option<ImageFiltersInput<'a>>>>,
    pub not: Option<Box<ImageFiltersInput<'a>>>,
}

#[derive(cynic::InputObject, Debug, Default)]
pub struct StringFilterInput {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Option<String>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Option<String>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<StringFilterInput>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eqi: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub ne: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub nei: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_contains: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub containsi: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_containsi: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gt: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gte: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lte: Option<String>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_null: Option<bool>,

    #[cynic(rename = "in", skip_serializing_if = "Option::is_none")]
    pub in_: Option<Vec<Option<String>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<Vec<Option<String>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub between: Option<Vec<Option<String>>>,
}

#[derive(cynic::InputObject, Debug, Default)]
pub struct BooleanFilterInput {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Option<bool>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Option<bool>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<BooleanFilterInput>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eq: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eqi: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub ne: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub nei: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub contains: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_contains: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub containsi: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_containsi: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gt: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gte: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lt: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lte: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_null: Option<bool>,

    #[cynic(rename = "in", skip_serializing_if = "Option::is_none")]
    pub in_: Option<Vec<Option<bool>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<Vec<Option<bool>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub between: Option<Vec<Option<bool>>>,
}
