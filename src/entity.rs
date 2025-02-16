use crate::models::{Advice, AdviceArticle, ExternalMedia, LostCat};

use crate::queries::advice::{ListAdvice, ListAdviceVariables};
use crate::queries::advice_article::{GetAdviceArticle, GetAdviceArticleVariables};
use crate::queries::external_media::{ListExternalMedia, ListExternalMediaVariables};
use crate::queries::{
    adopted_cat::{ListAdoptedCat, ListAdoptedCatVariables},
    announcement::{ListAnnouncements, ListAnnouncementsVariables},
    announcement_article::{GetArticle, GetArticleVariables},
    cat::{GetCat, GetCatVariables, ListCat, ListCatVariables},
    found_cat::{ListFoundCat, ListFoundCatVariables},
    looking_for_home::{ListLookingForAdoptionQuery, ListLookingForAdoptionVariables},
    lost_cat::{ListLostCat, ListLostCatVariables},
    virtual_cat::{ListSupporterWithCats, ListSupporterWithCatsVariables},
};
use crate::{
    get_client, AdoptedCat, Announcement, Article, Cat, CynicRequestSnafu, EnvVarMissingSnafu,
    Error, FoundCat, LookingForHomeCat, MissingAttributeSnafu, Paged, Supporter,
};
use cynic::http::ReqwestBlockingExt;
use cynic::{GraphQlResponse, Operation, QueryBuilder};
use snafu::{OptionExt, ResultExt};
use std::env;

#[cfg(feature = "elixir_support")]
pub trait SingularEntity: Sized + rustler::Encoder {
    type Variables: serde::Serialize;
    type QueryType: QueryBuilder<Self::Variables> + 'static + serde::de::DeserializeOwned; // Divine Intellect
    type ResponseData;

    fn build_query(vars: Self::Variables) -> Operation<Self::QueryType, Self::Variables> {
        Self::QueryType::build(vars)
    }
    fn extract_singular_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Self, Error>;
}

#[cfg(not(feature = "elixir_support"))]
pub trait SingularEntity: Sized {
    type Variables: serde::Serialize;
    type QueryType: QueryBuilder<Self::Variables> + 'static + serde::de::DeserializeOwned; // Divine Intellect
    type ResponseData;

    fn build_query(vars: Self::Variables) -> Operation<Self::QueryType, Self::Variables> {
        Self::QueryType::build(vars)
    }

    fn extract_singular_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Self, Error>;
}
pub fn get_entity<T: SingularEntity>(variables: T::Variables) -> Result<T, Error> {
    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;

    let vars_str = serde_json::to_string(&variables);

    let operation: Operation<T::QueryType, T::Variables> = T::build_query(variables);

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
    T::extract_singular_data(response)
}

#[cfg(feature = "elixir_support")]
pub trait ListableEntity: Sized + rustler::Encoder {
    type Variables: serde::Serialize;
    type QueryType: QueryBuilder<Self::Variables> + 'static + serde::de::DeserializeOwned; // Divine Intellect
    type ResponseData;

    fn build_query(vars: Self::Variables) -> Operation<Self::QueryType, Self::Variables> {
        Self::QueryType::build(vars)
    }
    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error>;
}

#[cfg(not(feature = "elixir_support"))]
pub trait ListableEntity: Sized {
    type Variables: serde::Serialize;
    type QueryType: QueryBuilder<Self::Variables> + 'static + serde::de::DeserializeOwned; // Divine Intellect
    type ResponseData;

    fn build_query(vars: Self::Variables) -> Operation<Self::QueryType, Self::Variables> {
        Self::QueryType::build(vars)
    }

    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error>;
}

pub fn list_entity<T: ListableEntity>(variables: T::Variables) -> Result<Paged<T>, Error> {
    let endpoint = env::var("STRAPI_ENDPOINT").context(EnvVarMissingSnafu {})?;

    let vars_str = serde_json::to_string(&variables);

    let operation: Operation<T::QueryType, T::Variables> = T::build_query(variables);

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
    T::extract_paged_data(response)
}
impl ListableEntity for FoundCat {
    type QueryType = ListFoundCat;
    type Variables = ListFoundCatVariables<'static>;
    type ResponseData = GraphQlResponse<Self::QueryType>;

    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let source_cats = response_data
            .data
            .context(MissingAttributeSnafu {})?
            .found_cats
            .context(MissingAttributeSnafu {})?;

        let meta = source_cats.meta;

        let cats: Result<Vec<FoundCat>, Error> = source_cats
            .data
            .into_iter()
            .map(|cat_entity| {
                let id = cat_entity.id.map(|id| id.into_inner());
                cat_entity
                    .attributes
                    .context(MissingAttributeSnafu {})
                    .and_then(|cat| {
                        let FoundCat {
                            cat,
                            found_datetime,
                            special_signs,
                            found_location,
                            discovery_circumstances,
                            ..
                        } = cat.try_into()?;
                        Ok(FoundCat {
                            discovery_circumstances,
                            found_location,
                            special_signs,
                            id,
                            found_datetime,
                            cat,
                        })
                    })
            })
            .collect();
        Ok(Paged::new(meta.pagination, cats?))
    }
}
impl ListableEntity for LookingForHomeCat {
    type Variables = ListLookingForAdoptionVariables<'static>;
    type QueryType = ListLookingForAdoptionQuery;
    type ResponseData = GraphQlResponse<Self::QueryType>;
    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let source_cats = response_data
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

        Ok(Paged::new(meta.pagination, cats?))
    }
}
impl ListableEntity for LostCat {
    type Variables = ListLostCatVariables<'static>;
    type QueryType = ListLostCat;
    type ResponseData = GraphQlResponse<Self::QueryType>;
    fn extract_paged_data(response_data: Self::ResponseData) -> Result<Paged<Self>, Error> {
        let source_cats = response_data
            .data
            .context(MissingAttributeSnafu {})?
            .lost_cats
            .context(MissingAttributeSnafu {})?;
        let meta = source_cats.meta;
        let cats: Result<Vec<LostCat>, Error> = source_cats
            .data
            .into_iter()
            .map(|cat_entity| {
                let id = cat_entity.id.map(|id| id.into_inner());
                cat_entity
                    .attributes
                    .context(MissingAttributeSnafu {})
                    .and_then(|cat| {
                        let LostCat {
                            cat,
                            disappearance_circumstances,
                            during_medical_treatment,
                            disappearance_location,
                            disappearance_datetime,
                            special_signs,
                            ..
                        } = cat.try_into()?;
                        Ok(LostCat {
                            disappearance_datetime,
                            disappearance_location,
                            during_medical_treatment,
                            disappearance_circumstances,
                            special_signs,
                            id,
                            cat,
                        })
                    })
            })
            .collect();
        Ok(Paged::new(meta.pagination, cats?))
    }
}
impl ListableEntity for Cat {
    type QueryType = ListCat;
    type Variables = ListCatVariables<'static>;
    type ResponseData = GraphQlResponse<Self::QueryType>;
    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let source_cats = response_data
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
        Ok(Paged::new(meta.pagination, cats?))
    }
}
impl ListableEntity for Supporter {
    type QueryType = ListSupporterWithCats;
    type Variables = ListSupporterWithCatsVariables;
    type ResponseData = GraphQlResponse<Self::QueryType>;
    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let supporters = response_data
            .data
            .context(MissingAttributeSnafu {})?
            .supporters
            .context(MissingAttributeSnafu {})?;

        let meta = supporters.meta;

        let supporters: Result<Vec<Supporter>, Error> = supporters
            .data
            .into_iter()
            .map(|supporter_entity| {
                supporter_entity
                    .attributes
                    .context(MissingAttributeSnafu {})?
                    .try_into()
            })
            .collect();

        Ok(Paged::new(meta.pagination, supporters?))
    }
}

impl ListableEntity for AdoptedCat {
    type QueryType = ListAdoptedCat;
    type Variables = ListAdoptedCatVariables<'static>;
    type ResponseData = GraphQlResponse<Self::QueryType>;
    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let source_cats = response_data
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
                            adoption_date,
                            cat,
                            caretaker,
                            ..
                        } = adopted_cat.try_into()?;
                        Ok(AdoptedCat {
                            id,
                            caretaker,
                            cat,
                            adoption_date,
                        })
                    })
            })
            .collect();

        Ok(Paged::new(meta.pagination, adopted_cats?))
    }
}
impl ListableEntity for ExternalMedia {
    type QueryType = ListExternalMedia;
    type Variables = ListExternalMediaVariables<'static>;
    type ResponseData = GraphQlResponse<Self::QueryType>;

    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let source_external_media = response_data
            .data
            .context(MissingAttributeSnafu {})?
            .external_medias
            .context(MissingAttributeSnafu {})?;

        let meta = source_external_media.meta;

        let external_media: Result<Vec<ExternalMedia>, Error> = source_external_media
            .data
            .into_iter()
            .map(|media_entity| {
                let id = media_entity.id.map(|id| id.into_inner());
                media_entity
                    .attributes
                    .context(MissingAttributeSnafu {})
                    .map(|media| ExternalMedia { id, ..media.into() })
            })
            .collect();

        Ok(Paged::new(meta.pagination, external_media?))
    }
}

impl ListableEntity for Advice {
    type QueryType = ListAdvice;
    type Variables = ListAdviceVariables<'static>;
    type ResponseData = GraphQlResponse<Self::QueryType>;

    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let source_advices = response_data
            .data
            .context(MissingAttributeSnafu {})?
            .advices
            .context(MissingAttributeSnafu {})?;

        let meta = source_advices.meta;

        let advices: Result<Vec<Advice>, Error> = source_advices
            .data
            .into_iter()
            .map(|advice_entity| {
                let id = advice_entity.id.map(|id| id.into_inner());
                advice_entity
                    .attributes
                    .context(MissingAttributeSnafu {})
                    .map(|advice| Advice {
                        id,
                        ..advice.into()
                    })
            })
            .collect();

        Ok(Paged::new(meta.pagination, advices?))
    }
}

impl ListableEntity for Announcement {
    type QueryType = ListAnnouncements;
    type Variables = ListAnnouncementsVariables<'static>;
    type ResponseData = GraphQlResponse<Self::QueryType>;

    fn extract_paged_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Paged<Self>, Error> {
        let source_announcements = response_data
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

        Ok(Paged::new(meta.pagination, announcements?))
    }
}

impl SingularEntity for AdviceArticle {
    type Variables = GetAdviceArticleVariables;
    type QueryType = GetAdviceArticle;
    type ResponseData = GraphQlResponse<Self::QueryType>;

    fn extract_singular_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<AdviceArticle, Error> {
        let source_advice = response_data
            .data
            .context(MissingAttributeSnafu {})?
            .advice
            .context(MissingAttributeSnafu {})?
            .data
            .context(MissingAttributeSnafu {})?
            .attributes
            .context(MissingAttributeSnafu {})?;

        source_advice.try_into()
    }
}
impl SingularEntity for Article {
    type Variables = GetArticleVariables;
    type QueryType = GetArticle;
    type ResponseData = GraphQlResponse<Self::QueryType>;

    fn extract_singular_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Article, Error> {
        let source_announcement = response_data
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
}

impl SingularEntity for Cat {
    type Variables = GetCatVariables;
    type QueryType = GetCat;
    type ResponseData = GraphQlResponse<Self::QueryType>;
    fn extract_singular_data(
        response_data: GraphQlResponse<Self::QueryType>,
    ) -> Result<Self, Error> {
        let cat_entity = response_data
            .data
            .context(MissingAttributeSnafu {})?
            .cat
            .context(MissingAttributeSnafu {})?
            .data
            .context(MissingAttributeSnafu {})?;

        let source_cat = cat_entity.attributes.context(MissingAttributeSnafu {})?;

        Ok(Cat {
            id: cat_entity.id.map(|id| id.into_inner()),
            ..source_cat.into()
        })
    }
}
