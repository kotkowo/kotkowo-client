pub use crate::queries::commons::*;
use crate::schema;

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
    pub age: Option<Age>,
    pub castrated: Option<bool>,
    #[cynic(rename = "cat_tags")]
    pub cat_tags: Option<CatTagRelationResponseCollection>,
    #[cynic(rename = "chip_number")]
    pub chip_number: Option<String>,
    pub color: Color,
    #[cynic(rename = "contact_informations")]
    pub contact_informations: Option<ContactInformationRelationResponseCollection>,
    #[cynic(rename = "is_dead")]
    pub is_dead: bool,
    #[cynic(rename = "medical_status")]
    pub medical_status: Option<MedicalStatus>,
    pub name: Option<String>,
    pub sex: Sex,
    pub slug: String,
    pub healthy: Option<bool>,
    #[cynic(rename = "fiv_felv")]
    pub fiv_felv: Option<FivFelv>,
    #[cynic(rename = "description_heading")]
    pub description_heading: Option<String>,
    pub description: Option<String>,
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
    #[cynic(skip_serializing_if = "Option::is_none")]
    #[cynic(rename = "description_heading")]
    pub description_heading: Option<StringFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<StringFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub sex: Option<StringFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub age: Option<StringFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    #[cynic(rename = "medical_status")]
    pub medical_status: Option<StringFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    #[cynic(rename = "fiv_felv")]
    pub fiv_felv: Option<StringFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub castrated: Option<BooleanFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub healthy: Option<BooleanFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    #[cynic(rename = "cat_tags")]
    pub cat_tags: Option<CatTagFiltersInput<'a>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub color: Option<StringFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    #[cynic(rename = "is_dead")]
    pub is_dead: Option<BooleanFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    #[cynic(rename = "contact_informations")]
    pub contact_informations: Option<ContactInformationFiltersInput<'a>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    #[cynic(rename = "chip_number")]
    pub chip_number: Option<StringFilterInput>,
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
#[derive(cynic::InputObject, Debug, Default)]
pub struct AdoptedCatFiltersInput<'a> {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdfilterInput<'a>>,
    #[cynic(rename = "adoption_date")]
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub adoption_date: Option<DateTimeFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Box<CatFiltersInput<'a>>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTimeFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTimeFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTimeFilterInput>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Option<AdoptedCatFiltersInput<'a>>>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Option<AdoptedCatFiltersInput<'a>>>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<AdoptedCatFiltersInput<'a>>>,
}
