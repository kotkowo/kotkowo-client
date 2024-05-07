#![allow(non_snake_case)] // please don't abuse, it's for rustler's generated atoms

pub use crate::queries::commons::*;
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

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
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

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
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

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
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
