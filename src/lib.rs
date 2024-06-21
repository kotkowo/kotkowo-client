mod errors;
mod models;
mod options;
mod queries;
mod schema;

pub use errors::*;
use models::LookingForHomeCat;
pub use models::{AdoptedCat, Age, Announcement, Article, Cat, Color, Paged, Sex};
pub use options::{AnnouncementFilter, BetweenDateTime, CatFilter, Options};
pub use queries::commons::PaginationArg;

use queries::{cat::CatFiltersInput, commons::DateTime};
use snafu::{OptionExt, ResultExt};
use std::env;

use crate::queries::commons::{BooleanFilterInput, StringFilterInput};

// this should work fine but breaks rust-analyzer
// pub type Result<T, E = Error> = std::result::Result<T, E>;
pub fn get_announcement_article(announcement_id: String) -> Result<Article, Error> {
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::announcement_article::{GetArticle, GetArticleVariables};

    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;
    let id: cynic::Id = announcement_id.into();
    let vars = GetArticleVariables { id: &id };
    let vars_str = serde_json::to_string(&vars);
    let operation = GetArticle::build(vars);
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

    let source_announcement = response
        .data
        .context(MissingAttributeSnafu {})?
        .announcement
        .context(MissingAttributeSnafu {})?
        .data
        .context(MissingAttributeSnafu {})?
        .attributes
        .context(MissingAttributeSnafu {})?;

    source_announcement.try_into()
}
pub fn list_announcement(
    options: Options<AnnouncementFilter>,
) -> Result<Paged<Announcement>, Error> {
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::announcement::{ListAnnouncements, ListAnnouncementsVariables};

    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;

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
    let vars_str = serde_json::to_string(&vars);
    let operation = ListAnnouncements::build(vars);
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

    let source_announcements = response
        .data
        .context(MissingAttributeSnafu {})?
        .announcements
        .context(MissingAttributeSnafu {})?;

    let meta = source_announcements.meta;

    let announcements: Result<Vec<Announcement>, Error> = source_announcements
        .data
        .into_iter()
        .map(|announcement_entity| {
            let id = announcement_entity.id.map(|id| id.into_inner());
            announcement_entity
                .attributes
                .context(MissingAttributeSnafu {})
                .map(|announcement| Announcement {
                    id,
                    ..announcement.into()
                })
        })
        .collect();

    let page: Paged<Announcement> = Paged::new(meta.pagination, announcements?);

    Ok(page)
}

pub fn get_cat(id: String) -> Result<Cat, Error> {
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::cat::{GetCat, GetCatVariables};

    let id: cynic::Id = id.into();
    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;
    let operation = GetCat::build(GetCatVariables { id: &id });
    let client = get_client()?;
    let response = client
        .post(endpoint)
        .run_graphql(operation)
        .context(CynicRequestSnafu {})?;

    if let Some(err) = response.errors {
        let message = format!("{:?}", err).to_string();
        return Err(Error::RequestResultedInError { message });
    }

    let cat_entity = response
        .data
        .context(MissingAttributeSnafu {})?
        .cat
        .context(MissingAttributeSnafu {})?
        .data
        .context(MissingAttributeSnafu {})?;

    let source_cat = cat_entity.attributes.context(MissingAttributeSnafu {})?;

    let cat: Cat = Cat {
        id: cat_entity.id.map(|id| id.into_inner()),
        ..source_cat.into()
    };

    Ok(cat)
}

pub fn list_adopted_cat(
    options: Options<CatFilter>,
    between_dates: Option<BetweenDateTime>,
) -> Result<Paged<AdoptedCat>, Error> {
    use crate::queries::adopted_cat::AdoptedCatQueryVariables;
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::adopted_cat::AdoptedCatQuery;

    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;

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
    let vars = AdoptedCatQueryVariables {
        cat: filters,
        pagination,
        sort,
        between,
    };

    // stored in case needed for error message
    let vars_str = serde_json::to_string(&vars);

    let operation = AdoptedCatQuery::build(vars);
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
        .adopted_cats
        .context(MissingAttributeSnafu {})?;

    let meta = source_cats.meta;

    let adopted_cats: Result<Vec<AdoptedCat>, Error> = source_cats
        .data
        .into_iter()
        .map(|cat_entity| {
            let id = cat_entity.id.map(|id| id.into_inner());
            cat_entity
                .attributes
                .context(MissingAttributeSnafu {})
                .and_then(|adopted_cat| {
                    let AdoptedCat {
                        adoption_date, cat, ..
                    } = adopted_cat.try_into()?;
                    Ok(AdoptedCat {
                        id,
                        cat,
                        adoption_date,
                    })
                })
        })
        .collect();

    let page: Paged<AdoptedCat> = Paged::new(meta.pagination, adopted_cats?);

    Ok(page)
}

pub fn get_cat_by_slug(slug: String) -> Result<Cat, Error> {
    use crate::queries::cat::ListCatVariables;
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::cat::ListCat;

    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;
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

    // stored in case needed for error message
    let vars_str = serde_json::to_string(&vars);

    let operation = ListCat::build(vars);
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
        .cats
        .context(MissingAttributeSnafu {})?;

    let cats: Result<Vec<Cat>, Error> = source_cats
        .data
        .into_iter()
        .map(|cat_entity| {
            let id = cat_entity.id.map(|id| id.into_inner());
            cat_entity
                .attributes
                .context(MissingAttributeSnafu {})
                .map(|cat| Cat { id, ..cat.into() })
        })
        .collect();

    let cats: Vec<Cat> = cats?;
    assert!(cats.len() < 2);
    let cat: Cat = cats
        .into_iter()
        .next()
        .context(NotFoundSnafu { key: key.unwrap() })?;

    Ok(cat)
}

pub fn list_looking_for_adoption_cat(
    options: Options<CatFilter>,
    owned_by_kotkowo: Option<bool>,
) -> Result<Paged<LookingForHomeCat>, Error> {
    use queries::looking_for_home::{ListLookingForAdoptionQuery, ListLookingForAdoptionVariables};

    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;

    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;

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

    // stored in case needed for error message
    let vars_str = serde_json::to_string(&vars);

    let operation = ListLookingForAdoptionQuery::build(vars);
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
        .looking_for_adoption_cats
        .context(MissingAttributeSnafu {})?;

    let meta = source_cats.meta;

    let cats: Result<Vec<LookingForHomeCat>, Error> = source_cats
        .data
        .into_iter()
        .map(|cat_entity| {
            let id = cat_entity.id.map(|id| id.into_inner());
            cat_entity
                .attributes
                .context(MissingAttributeSnafu {})
                .and_then(|cat| {
                    let LookingForHomeCat { cat, caretaker, .. } = cat.try_into()?;
                    Ok(LookingForHomeCat { id, cat, caretaker })
                })
        })
        .collect();

    let page: Paged<LookingForHomeCat> = Paged::new(meta.pagination, cats?);

    Ok(page)
}

pub fn list_cat(options: Options<CatFilter>) -> Result<Paged<Cat>, Error> {
    use crate::queries::cat::ListCatVariables;
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::cat::ListCat;

    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;

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

    // stored in case needed for error message
    let vars_str = serde_json::to_string(&vars);

    let operation = ListCat::build(vars);
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
        .cats
        .context(MissingAttributeSnafu {})?;

    let meta = source_cats.meta;

    let cats: Result<Vec<Cat>, Error> = source_cats
        .data
        .into_iter()
        .map(|cat_entity| {
            let id = cat_entity.id.map(|id| id.into_inner());
            cat_entity
                .attributes
                .context(MissingAttributeSnafu {})
                .map(|cat| Cat { id, ..cat.into() })
        })
        .collect();

    let page: Paged<Cat> = Paged::new(meta.pagination, cats?);

    Ok(page)
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
        list_cat, list_looking_for_adoption_cat, Options,
    };

    #[test]
    fn list_announcement_test() {
        let paged = list_announcement(Options::default());
        assert!(paged.is_ok());
    }
    #[test]
    fn get_announcement_article_test() {
        let article = get_announcement_article("1".to_string());
        assert!(article.is_ok());
    }
    #[test]
    fn list_adopted_cat_test() {
        let paged = list_adopted_cat(Options::default(), None);
        assert!(paged.is_ok());
    }
    #[test]
    fn list_cat_test() {
        let opts = Options::default();
        let paged = list_cat(opts);
        println!("{:?}", paged);
        assert!(paged.is_ok())
    }
    #[test]
    fn list_lfh_cat_test() {
        let opts = Options::default();
        let paged = list_looking_for_adoption_cat(opts, None);
        println!("{:?}", paged);
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
        let slug = "cat-4";
        let cat = get_cat_by_slug(slug.to_string());
        println!("{:?}", cat);
        assert!(cat.is_ok())
    }
}
