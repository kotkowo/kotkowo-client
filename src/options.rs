use crate::{
    queries::cat::{BooleanFilterInput, CatFiltersInput, CatTagFiltersInput, StringFilterInput},
    Age, Color, PaginationArg, Sex,
};

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Opts"
)]
pub struct Options<
    #[cfg(not(feature = "elixir_support"))] F,
    #[cfg(feature = "elixir_support")] F: rustler::Encoder + for<'a> rustler::Decoder<'a>,
> {
    pub filter: Option<F>,
    pub pagination: Option<PaginationArg>,
    pub sort: Vec<String>,
}

#[derive(Debug, Default)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Cat.Filter"
)]
pub struct CatFilter {
    sex: Option<Sex>,
    age: Option<Age>,
    color: Option<Color>,
    castrated: Option<bool>,
    tags: Option<Vec<String>>,
}

impl<'a> From<CatFilter> for CatFiltersInput<'a> {
    fn from(val: CatFilter) -> Self {
        let sex_eq: Option<String> = val.sex.map(|sex| format!("{:?}", sex));
        let age_eq: Option<String> = val.age.map(|age| format!("{:?}", age));
        let color_eq: Option<String> = val.color.map(|color| format!("{:?}", color));
        let tags_filters: Option<Vec<Option<CatTagFiltersInput>>> = val.tags.map(|tags| {
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
            castrated: Some(BooleanFilterInput {
                eq: val.castrated,
                ..BooleanFilterInput::default()
            }),
            color: Some(StringFilterInput {
                eq: color_eq,
                ..StringFilterInput::default()
            }),
            age: Some(StringFilterInput {
                eq: age_eq,
                ..StringFilterInput::default()
            }),
            sex: Some(StringFilterInput {
                eqi: sex_eq,
                ..StringFilterInput::default()
            }),
            cat_tags: Some(CatTagFiltersInput {
                or: tags_filters,
                ..CatTagFiltersInput::default()
            }),
            ..CatFiltersInput::default()
        }
    }
}
