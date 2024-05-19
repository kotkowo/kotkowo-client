mod errors;
mod models;
mod options;
mod queries;
mod schema;

pub use errors::*;
pub use models::{Age, Announcement, Article, Cat, Color, Paged, Sex};
pub use options::{AnnouncementFilter, CatFilter, Options};
pub use queries::commons::PaginationArg;

use queries::cat::CatFiltersInput;
use snafu::{OptionExt, ResultExt};

// this should work fine but breaks rust-analyzer
// pub type Result<T, E = Error> = std::result::Result<T, E>;
pub fn get_announcement_article(announcement_id: String) -> Result<Article, Error> {
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::announcement_article::{GetArticle, GetArticleVariables};

    let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";
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

    let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";

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
    let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";
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

pub fn list_cat(options: Options<CatFilter>) -> Result<Paged<Cat>, Error> {
    use crate::queries::cat::ListCatVariables;
    use cynic::http::ReqwestBlockingExt;
    use cynic::QueryBuilder;
    use queries::cat::ListCat;

    let endpoint = "https://kotkowo-admin.ravensiris.xyz/graphql";

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
    let api_key = std::env::var("STRAPI_KEY").context(EnvVarMissingSnafu {})?;
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
    use crate::{get_announcement_article, list_announcement, Options};

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
}
