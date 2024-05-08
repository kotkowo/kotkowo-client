use crate::{
    queries::cat::{CatFiltersInput, CatTagFiltersInput},
    queries::commons::{BooleanFilterInput, StringFilterInput},
    Age, Color, PaginationArg, Sex,
};

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Opts"
)]
pub struct Options<
    #[cfg(not(feature = "elixir_support"))] F,
    #[cfg(feature = "elixir_support")] F: rustler::Encoder,
> {
    pub filter: Option<F>,
    pub pagination: Option<PaginationArg>,
    pub sort: Vec<String>,
}

#[derive(Debug)]
#[cfg_attr(feature = "elixir_support", derive(rustler::NifTaggedEnum))]
enum Filter<
    #[cfg(not(feature = "elixir_support"))] T,
    #[cfg(feature = "elixir_support")] T: rustler::Encoder,
> {
    Equals(T),
    EqualsCI(T),
    Contains(T),
    ContainsCI(T),
    Or(Vec<T>),
    In(Vec<T>),
}

impl<
        #[cfg(not(feature = "elixir_support"))] T: ToString,
        #[cfg(feature = "elixir_support")] T: ToString + rustler::Encoder,
    > From<Filter<T>> for StringFilterInput
{
    fn from(value: Filter<T>) -> Self {
        match value {
            Filter::Equals(value) => StringFilterInput {
                eq: Some(value.to_string()),
                ..StringFilterInput::default()
            },
            Filter::EqualsCI(value) => StringFilterInput {
                eqi: Some(value.to_string()),
                ..StringFilterInput::default()
            },
            Filter::Contains(value) => StringFilterInput {
                contains: Some(value.to_string()),
                ..StringFilterInput::default()
            },
            Filter::ContainsCI(value) => StringFilterInput {
                containsi: Some(value.to_string()),
                ..Default::default()
            },
            Filter::Or(values) => StringFilterInput {
                or: Some(values.into_iter().map(|v| Some(v.to_string())).collect()),
                ..StringFilterInput::default()
            },
            Filter::In(values) => StringFilterInput {
                in_: Some(values.into_iter().map(|v| Some(v.to_string())).collect()),
                ..StringFilterInput::default()
            },
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Cat.Filter"
)]
pub struct CatFilter {
    sex: Option<Filter<Sex>>,
    age: Option<Filter<Age>>,
    color: Option<Filter<Color>>,
    castrated: Option<bool>,
    tags: Option<Vec<String>>,
    name: Option<Filter<String>>,
}

impl<'a> From<CatFilter> for CatFiltersInput<'a> {
    fn from(value: CatFilter) -> Self {
        let CatFilter {
            name,
            tags,
            castrated,
            color,
            age,
            sex,
        } = value;
        let tags: Option<Vec<Option<CatTagFiltersInput>>> = tags.map(|tags| {
            tags.into_iter()
                .map(|tag| {
                    Some(CatTagFiltersInput {
                        text: Some(StringFilterInput {
                            containsi: Some(tag),
                            ..StringFilterInput::default()
                        }),
                        ..CatTagFiltersInput::default()
                    })
                })
                .collect()
        });

        CatFiltersInput {
            name: name.map(|v| v.into()),
            color: color.map(|v| v.into()),
            age: age.map(|v| v.into()),
            sex: sex.map(|v| v.into()),
            cat_tags: Some(CatTagFiltersInput {
                or: tags,
                ..CatTagFiltersInput::default()
            }),
            castrated: Some(BooleanFilterInput {
                eq: castrated,
                ..BooleanFilterInput::default()
            }),
            ..CatFiltersInput::default()
        }
    }
}
