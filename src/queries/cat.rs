use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct GetCatVariables<'a> {
    pub id: &'a cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ListCatVariables<'a> {
    pub filters: CatFiltersInput<'a>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ListCatVariables")]
pub struct ListCat {
    #[arguments(filters: $filters)]
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
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Cat"
)]
pub struct Cat {
    pub name: String,
    pub slug: String,
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
