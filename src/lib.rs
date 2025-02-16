#![feature(associated_type_defaults)]
mod errors;
mod models;
mod options;
mod queries;
mod schema;

pub use errors::*;
pub use models::{
    AdoptedCat, Advice, Age, Announcement, Article, Cat, Color, ExternalMedia, FoundCat,
    LookingForHomeCat, LostCat, Paged, Sex, Supporter,
};
pub use options::{
    AdviceFilter, AnnouncementFilter, BetweenDateTime, CatFilter, ExternalMediaFilter, Options,
};
pub use queries::commons::{ContactInformation, PaginationArg};

use queries::{
    cat::CatFiltersInput, commons::DateTime, looking_for_home::ListLookingForAdoptionVariables,
};
use snafu::{OptionExt, ResultExt};
use std::env;

use crate::{
    entity::{get_entity, list_entity},
    models::AdviceArticle,
    queries::{
        commons::{BooleanFilterInput, StringFilterInput},
        external_media::ListExternalMediaVariables,
        found_cat::ListFoundCatVariables,
        lost_cat::ListLostCatVariables,
    },
};
mod entity;

// this should work fine but breaks rust-analyzer
// pub type Result<T, E = Error> = std::result::Result<T, E>;
pub fn get_announcement_article(announcement_id: String) -> Result<Article, Error> {
    use queries::announcement_article::GetArticleVariables;
    let id: cynic::Id = announcement_id.into();
    let vars = GetArticleVariables { id };
    get_entity::<Article>(vars)
}
pub fn get_advice_article(advice_id: String) -> Result<Article, Error> {
    use queries::advice_article::GetAdviceArticleVariables;
    let id: cynic::Id = advice_id.into();
    let vars = GetAdviceArticleVariables { id };
    Ok(get_entity::<AdviceArticle>(vars)?.into())
}
pub fn list_external_media(
    options: Options<ExternalMediaFilter>,
) -> Result<Paged<ExternalMedia>, Error> {
    let pagination = options.pagination;
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };

    let vars = ListExternalMediaVariables {
        filters: None,
        pagination,
        sort,
    };
    list_entity::<ExternalMedia>(vars)
}
pub fn list_announcement(
    options: Options<AnnouncementFilter>,
) -> Result<Paged<Announcement>, Error> {
    use queries::announcement::ListAnnouncementsVariables;

    let pagination = options.pagination;
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };

    let vars = ListAnnouncementsVariables {
        filters: None,
        pagination,
        sort,
    };
    list_entity::<Announcement>(vars)
}
pub fn list_advice(options: Options<AdviceFilter>) -> Result<Paged<Advice>, Error> {
    use queries::advice::ListAdviceVariables;

    let pagination = options.pagination;
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };

    let vars = ListAdviceVariables {
        filters: None,
        pagination,
        sort,
    };
    list_entity::<Advice>(vars)
}

pub fn get_cat(id: String) -> Result<Cat, Error> {
    use queries::cat::GetCatVariables;

    let id: cynic::Id = id.into();
    get_entity::<Cat>(GetCatVariables { id })
}

pub fn list_adopted_cat(
    options: Options<CatFilter>,
    between_dates: Option<BetweenDateTime>,
) -> Result<Paged<AdoptedCat>, Error> {
    use crate::queries::adopted_cat::ListAdoptedCatVariables;

    let filters: CatFiltersInput = options.filter.map_or_else(
        || CatFiltersInput {
            is_dead: Some(BooleanFilterInput {
                eq: Some(false),
                ..BooleanFilterInput::default()
            }),
            ..CatFiltersInput::default()
        },
        |filter| CatFiltersInput {
            is_dead: Some(BooleanFilterInput {
                eq: Some(false),
                ..BooleanFilterInput::default()
            }),
            ..filter.into()
        },
    );
    let pagination = options.pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };

    let between: Option<Vec<Option<DateTime>>> = between_dates.map(|dates| dates.into());
    let vars = ListAdoptedCatVariables {
        cat: filters,
        pagination,
        sort,
        between,
    };

    list_entity::<AdoptedCat>(vars)
}

impl CatFiltersInput<'_> {
    fn from_slug(slug: String) -> Self {
        CatFiltersInput {
            slug: Some(StringFilterInput {
                eq: Some(slug),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
pub fn get_found_cat_by_slug(slug: String) -> Result<FoundCat, Error> {
    let key = serde_json::to_string(&slug);
    let filters = CatFiltersInput::from_slug(slug);
    let pagination = PaginationArg::default();

    let vars = ListFoundCatVariables {
        filters,
        pagination,
        sort: None,
    };
    let cats = list_entity::<FoundCat>(vars)?.items;
    assert!(cats.len() < 2);
    cats.into_iter()
        .next()
        .context(NotFoundSnafu { key: key.unwrap() })
}
pub fn get_lost_cat_by_slug(slug: String) -> Result<LostCat, Error> {
    let key = serde_json::to_string(&slug);
    let filters = CatFiltersInput::from_slug(slug);
    let pagination = PaginationArg::default();

    let vars = ListLostCatVariables {
        filters,
        pagination,
        sort: None,
    };
    let cats = list_entity::<LostCat>(vars)?.items;
    assert!(cats.len() < 2);
    cats.into_iter()
        .next()
        .context(NotFoundSnafu { key: key.unwrap() })
}

pub fn get_lfh_cat_by_slug(slug: String) -> Result<LookingForHomeCat, Error> {
    let key = serde_json::to_string(&slug);
    let filters = CatFiltersInput::from_slug(slug);
    let pagination = PaginationArg::default();

    let vars = ListLookingForAdoptionVariables {
        filters,
        pagination,
        owned_by_kotkowo: None,
        sort: None,
    };
    let cats = list_entity::<LookingForHomeCat>(vars)?.items;
    assert!(cats.len() < 2);
    cats.into_iter()
        .next()
        .context(NotFoundSnafu { key: key.unwrap() })
}

pub fn get_cat_by_slug(slug: String) -> Result<Cat, Error> {
    use crate::queries::cat::ListCatVariables;

    let key = serde_json::to_string(&slug);

    let filters: CatFiltersInput = CatFiltersInput {
        slug: Some(StringFilterInput {
            eq: Some(slug),
            ..Default::default()
        }),
        ..Default::default()
    };

    let pagination = PaginationArg::default();

    let vars = ListCatVariables {
        filters,
        pagination,
        sort: None,
    };

    let cats: Vec<Cat> = list_entity::<Cat>(vars)?.items;

    assert!(cats.len() < 2);
    let cat: Cat = cats
        .into_iter()
        .next()
        .context(NotFoundSnafu { key: key.unwrap() })?;

    Ok(cat)
}

pub fn list_supporters_with_virtual_cats(
    pagination: Option<PaginationArg>,
    sort: Option<Vec<String>>,
) -> Result<Paged<Supporter>, Error> {
    use queries::virtual_cat::ListSupporterWithCatsVariables;

    let pagination = pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = sort.map(|s| s.into_iter().map(Some).collect());
    let vars = ListSupporterWithCatsVariables { pagination, sort };

    list_entity::<Supporter>(vars)
}

pub fn list_lost_cat(options: Options<CatFilter>) -> Result<Paged<LostCat>, Error> {
    use queries::lost_cat::ListLostCatVariables;
    let filters: CatFiltersInput = options
        .filter
        .map_or_else(CatFiltersInput::default, |filter| filter.into());
    let pagination = options.pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };
    let vars = ListLostCatVariables {
        filters,
        pagination,
        sort,
    };
    list_entity::<LostCat>(vars)
}

pub fn list_found_cat(options: Options<CatFilter>) -> Result<Paged<FoundCat>, Error> {
    use queries::found_cat::ListFoundCatVariables;

    let filters: CatFiltersInput = options
        .filter
        .map_or_else(CatFiltersInput::default, |filter| filter.into());
    let pagination = options.pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };
    let vars = ListFoundCatVariables {
        filters,
        pagination,
        sort,
    };

    list_entity::<FoundCat>(vars)
}

pub fn list_looking_for_adoption_cat(
    options: Options<CatFilter>,
    owned_by_kotkowo: Option<bool>,
) -> Result<Paged<LookingForHomeCat>, Error> {
    use queries::looking_for_home::ListLookingForAdoptionVariables;

    let filters: CatFiltersInput = options
        .filter
        .map_or_else(CatFiltersInput::default, |filter| filter.into());
    let pagination = options.pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };
    let vars = ListLookingForAdoptionVariables {
        owned_by_kotkowo,
        filters,
        pagination,
        sort,
    };

    list_entity::<LookingForHomeCat>(vars)
}

pub fn list_virtual_cat(options: Options<CatFilter>) -> Result<Paged<Cat>, Error> {
    use crate::queries::virtual_cat::ListVirtualCatVariables;
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::virtual_cat::ListVirtualCat;

    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;

    let filters: CatFiltersInput = options
        .filter
        .map_or_else(CatFiltersInput::default, |filter| filter.into());
    let pagination = options.pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };
    let vars = ListVirtualCatVariables {
        filters,
        pagination,
        sort,
    };

    let vars_str = serde_json::to_string(&vars);

    let operation = ListVirtualCat::build(vars);
    let query = operation.query.clone();
    let client = get_client()?;
    let response = client
        .post(endpoint)
        .run_graphql(operation)
        .context(CynicRequestSnafu {})?;

    if let Some(err) = response.errors {
        let message = format!(
            "Variables:\n{}\nGraphQL:\n{}\nError:\n{:?}",
            vars_str.unwrap(),
            query,
            err
        )
        .to_string();

        return Err(Error::RequestResultedInError { message });
    }

    let source_cats = response
        .data
        .context(MissingAttributeSnafu {})?
        .virtual_cats
        .context(MissingAttributeSnafu {})?;

    let meta = source_cats.meta;

    let cats: Result<Vec<Cat>, Error> = source_cats
        .data
        .into_iter()
        .map(|cat_entity| {
            cat_entity
                .attributes
                .context(MissingAttributeSnafu {})
                .and_then(|virtual_cat| {
                    Ok(virtual_cat
                        .cat
                        .context(MissingAttributeSnafu {})?
                        .data
                        .context(MissingAttributeSnafu {})?
                        .attributes
                        .context(MissingAttributeSnafu {})?
                        .into())
                })
        })
        .collect();

    let page: Paged<Cat> = Paged::new(meta.pagination, cats?);

    Ok(page)
}

pub fn list_cat(options: Options<CatFilter>) -> Result<Paged<Cat>, Error> {
    use crate::queries::cat::ListCatVariables;

    let filters: CatFiltersInput = options
        .filter
        .map_or_else(CatFiltersInput::default, |filter| filter.into());
    let pagination = options.pagination.unwrap_or_default();
    let sort: Option<Vec<Option<String>>> = match options.sort {
        empty if empty.is_empty() => None,
        otherwise => Some(otherwise.into_iter().map(Some).collect()),
    };
    let vars = ListCatVariables {
        filters,
        pagination,
        sort,
    };

    list_entity::<Cat>(vars)
}

fn get_client() -> Result<reqwest::blocking::Client, Error> {
    let api_key = env::var("STRAPI_KEY").context(EnvVarMissingSnafu {})?;
    let mut headers = reqwest::header::HeaderMap::with_capacity(1);
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key)
            .parse()
            .context(InvalidHeaderValueSnafu {})?,
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .context(RequestSnafu {})?;

    Ok(client)
}

#[cfg(test)]
mod tests {
    use crate::{
        get_announcement_article, get_cat, get_cat_by_slug, list_adopted_cat, list_announcement,
        list_cat, list_external_media, list_found_cat, list_looking_for_adoption_cat,
        list_lost_cat, list_supporters_with_virtual_cats, list_virtual_cat, Options, PaginationArg,
    };

    #[test]
    fn list_announcement_test() {
        let paged = list_announcement(Options::default());
        assert!(paged.is_ok());
    }
    #[test]
    fn get_announcement_article_test() {
        let article = get_announcement_article("4".to_string());
        assert!(article.is_ok());
    }
    #[test]
    fn list_adopted_cat_test() {
        let paged = list_adopted_cat(Options::default(), None);
        assert!(paged.is_ok());
    }

    #[test]
    fn list_lost_cat_test() {
        let opts = Options::default();
        let paged = list_lost_cat(opts);
        assert!(paged.is_ok())
    }
    #[test]
    fn list_found_cat_test() {
        let opts = Options::default();
        let paged = list_found_cat(opts);
        assert!(paged.is_ok())
    }

    #[test]
    fn list_virtual_cat_test() {
        let opts = Options::default();
        let paged = list_virtual_cat(opts);
        assert!(paged.is_ok())
    }

    #[test]
    fn list_supporters_with_cats_test() {
        let pagination = PaginationArg {
            ..Default::default()
        };
        let paged = list_supporters_with_virtual_cats(Some(pagination), None);
        assert!(paged.is_ok())
    }
    #[test]
    fn list_cat_test() {
        let opts = Options::default();
        let paged = list_cat(opts);
        assert!(paged.is_ok())
    }
    #[test]
    fn list_lfh_cat_test() {
        let opts = Options::default();
        let paged = list_looking_for_adoption_cat(opts, None);
        assert!(paged.is_ok())
    }
    #[test]
    fn get_cat_test() {
        let slug = "1";
        let cat = get_cat(slug.to_string());
        assert!(cat.is_ok());
    }
    #[test]
    fn get_cat_by_slug_test() {
        let slug = "luna";
        let cat = get_cat_by_slug(slug.to_string());
        assert!(cat.is_ok())
    }
    #[test]
    fn get_external_media() {
        let opts = Options::default();
        let media = list_external_media(opts);
        assert!(media.is_ok());
    }
}
