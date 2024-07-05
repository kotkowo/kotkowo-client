use crate::{
    queries::{
        cat::{CatFiltersInput, CatTagFiltersInput},
        commons::{BooleanFilterInput, DateTime, StringFilterInput},
    },
    Age, Color, PaginationArg, Sex,
};

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.BetweenDateTime"
)]
pub struct BetweenDateTime {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

impl From<BetweenDateTime> for Vec<Option<DateTime>> {
    fn from(value: BetweenDateTime) -> Self {
        let BetweenDateTime { date_from, date_to } = value;
        vec![date_from.map(DateTime), date_to.map(DateTime)]
    }
}

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
#[allow(dead_code)]
#[cfg_attr(feature = "elixir_support", derive(rustler::NifTaggedEnum))]
enum Filter<
    #[cfg(not(feature = "elixir_support"))] T,
    #[cfg(feature = "elixir_support")] T: rustler::Encoder,
> {
    Equals(T),
    EqualsCI(T),
    Contains(T),
    ContainsCI(T),
    StartsWith(T),
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
            Filter::StartsWith(value) => StringFilterInput {
                starts_with: Some(value.to_string()),
                ..StringFilterInput::default()
            },
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
    module = "Kotkowo.Client.Announcement.Filter"
)]
pub struct AnnouncementFilter {}

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Cat.Filter"
)]
pub struct CatFilter {
    sex: Option<Filter<Sex>>,
    chip_number: Option<Filter<String>>,
    age: Option<Filter<Age>>,
    color: Option<Filter<Color>>,
    is_dead: Option<bool>,
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
            chip_number,
            color,
            is_dead,
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
            chip_number: chip_number.map(|v| v.into()),
            cat_tags: Some(CatTagFiltersInput {
                or: tags,
                ..CatTagFiltersInput::default()
            }),
            castrated: Some(BooleanFilterInput {
                eq: castrated,
                ..BooleanFilterInput::default()
            }),
            is_dead: Some(BooleanFilterInput {
                eq: is_dead,
                ..BooleanFilterInput::default()
            }),
            ..CatFiltersInput::default()
        }
    }
}
