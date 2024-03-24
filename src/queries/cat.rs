#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct GetCatVariables<'a> {
    pub id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ListCatVariables<'a> {
    pub filters: CatFiltersInput<'a>,
    pub pagination: PaginationArg,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListCatVariables")]
pub struct ListCat {
    #[arguments(filters: $filters, pagination: $pagination, sort: $sort)]
    pub cats: Option<CatEntityResponseCollection>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetCatVariables")]
pub struct GetCat {
    #[arguments(id: $id)]
    pub cat: Option<CatEntityResponse>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CatEntityResponseCollection {
    pub meta: ResponseCollectionMeta,
    pub data: Vec<CatEntity>,
}

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
pub struct CatEntityResponse {
    pub data: Option<CatEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CatEntity {
    pub id: Option<cynic::Id>,
    pub attributes: Option<Cat>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Cat {
    pub name: String,
    pub slug: String,
    pub sex: Sex,
    pub age: Age,
    #[cynic(rename = "medical_status")]
    pub medical_status: MedicalStatus,
    #[cynic(rename = "fiv_felv")]
    pub fiv_felv: FivFelv,
    pub healthy: bool,
    #[cynic(rename = "cat_tags")]
    pub cat_tags: Option<CatTagRelationResponseCollection>,
    #[cynic(rename = "description_heading")]
    pub description_heading: String,
    pub description: String,
    #[cynic(rename = "is_dead")]
    pub is_dead: bool,
    pub castrated: bool,
    pub color: Color,
    pub created_at: Option<DateTime>,
    pub published_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub images: Option<ImageRelationResponseCollection>,
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

#[derive(cynic::QueryFragment, Debug)]
pub struct CatTagRelationResponseCollection {
    pub data: Vec<CatTagEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CatTagEntity {
    pub attributes: Option<CatTag>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CatTag {
    pub text: String,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(feature = "elixir_support", derive(rustler::NifUnitEnum))]
#[cynic(graphql_type = "ENUM_CAT_AGE")]
pub enum Age {
    #[cynic(rename = "Junior")]
    Junior,
    #[cynic(rename = "Adult")]
    Adult,
    #[cynic(rename = "Senior")]
    Senior,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(feature = "elixir_support", derive(rustler::NifUnitEnum))]
#[cynic(graphql_type = "ENUM_CAT_COLOR")]
pub enum Color {
    #[cynic(rename = "Black")]
    Black,
    #[cynic(rename = "Gray")]
    Gray,
    #[cynic(rename = "Tricolor")]
    Tricolor,
    #[cynic(rename = "Patched")]
    Patched,
    #[cynic(rename = "Ginger")]
    Ginger,
    #[cynic(rename = "OtherColor")]
    OtherColor,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(feature = "elixir_support", derive(rustler::NifUnitEnum))]
#[cynic(graphql_type = "ENUM_CAT_FIV_FELV")]
pub enum FivFelv {
    #[cynic(rename = "Negative")]
    Negative,
    #[cynic(rename = "Positive")]
    Positive,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(feature = "elixir_support", derive(rustler::NifUnitEnum))]
#[cynic(graphql_type = "ENUM_CAT_MEDICAL_STATUS")]
pub enum MedicalStatus {
    #[cynic(rename = "TestedAndVaccinated")]
    TestedAndVaccinated,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(feature = "elixir_support", derive(rustler::NifUnitEnum))]
#[cynic(graphql_type = "ENUM_CAT_SEX")]
pub enum Sex {
    #[cynic(rename = "Male")]
    Male,
    #[cynic(rename = "Female")]
    Female,
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

#[derive(cynic::InputObject, Debug, Default)]
pub struct CatFiltersInput<'a> {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdfilterInput<'a>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub name: Option<StringFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub slug: Option<StringFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub images: Option<ImageFiltersInput<'a>>,

    #[cynic(
        rename = "description_heading",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_heading: Option<StringFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<StringFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sex: Option<StringFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub age: Option<StringFilterInput>,

    #[cynic(rename = "medical_status", skip_serializing_if = "Option::is_none")]
    pub medical_status: Option<StringFilterInput>,

    #[cynic(rename = "fiv_felv", skip_serializing_if = "Option::is_none")]
    pub fiv_felv: Option<StringFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub castrated: Option<BooleanFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub healthy: Option<BooleanFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none", rename = "cat_tags")]
    pub cat_tags: Option<CatTagFiltersInput<'a>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub color: Option<StringFilterInput>,

    #[cynic(rename = "is_dead", skip_serializing_if = "Option::is_none")]
    pub is_dead: Option<BooleanFilterInput>,

    #[cynic(rename = "adopted_cat", skip_serializing_if = "Option::is_none")]
    pub adopted_cat: Option<AdoptedCatFiltersInput<'a>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTimeFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTimeFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTimeFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Option<CatFiltersInput<'a>>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Option<CatFiltersInput<'a>>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<CatFiltersInput<'a>>>,
}

#[derive(cynic::InputObject, Debug, Default)]
pub struct CatTagFiltersInput<'a> {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdfilterInput<'a>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub text: Option<StringFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTimeFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTimeFilterInput>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Option<CatTagFiltersInput<'a>>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Option<CatTagFiltersInput<'a>>>>,

    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<CatTagFiltersInput<'a>>>,
}

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

#[derive(cynic::InputObject, Debug)]
pub struct AdoptedCatFiltersInput<'a> {
    pub id: Option<IdfilterInput<'a>>,
    #[cynic(rename = "adoption_date")]
    pub adoption_date: Option<DateTimeFilterInput>,
    pub created_at: Option<DateTimeFilterInput>,
    pub updated_at: Option<DateTimeFilterInput>,
    pub published_at: Option<DateTimeFilterInput>,
    pub and: Option<Vec<Option<AdoptedCatFiltersInput<'a>>>>,
    pub or: Option<Vec<Option<AdoptedCatFiltersInput<'a>>>>,
    pub not: Option<Box<AdoptedCatFiltersInput<'a>>>,
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
