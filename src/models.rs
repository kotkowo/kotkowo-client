use crate::errors::{Error, MissingAttributeSnafu};
use crate::queries::commons::{ContactInformation, DateTime};
use crate::queries::looking_for_home::LookingForAdoptionCat;
use crate::queries::{
    adopted_cat::AdoptedCat as SourceAdoptedCat,
    announcement::Announcement as SourceAnnouncement,
    announcement_article::{
        Announcement as SourceArticleAnnouncement, Article as SourceArticle, ArticleEntity,
        ArticleEntityResponse,
    },
    cat::Cat as SourceCat,
    commons::{UploadFile, UploadFileEntityResponse},
    external_media::ExternalMedia as SourceExternalMedia,
    found_cat::FoundCat as SourceFoundCat,
    lost_cat::LostCat as SourceLostCat,
    virtual_cat::Supporter as SourceSupporter,
};
pub use crate::queries::{
    cat::{Age, Color, FivFelv, MedicalStatus, Sex},
    commons::Pagination,
};
use snafu::OptionExt;

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Image"
)]
pub struct Image {
    pub id: Option<String>,
    pub url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub mime: String,
    pub name: String,
    pub preview_url: Option<String>,
    pub alternative_text: Option<String>,
}

impl TryFrom<UploadFileEntityResponse> for Image {
    type Error = Error;
    fn try_from(value: UploadFileEntityResponse) -> Result<Image, Error> {
        let data = value.data.context(MissingAttributeSnafu {})?;
        let attributes = data.attributes.context(MissingAttributeSnafu {})?;
        Ok(Image {
            id: data.id.map(|id| id.into_inner()),
            alternative_text: attributes.alternative_text,
            preview_url: attributes.preview_url,
            mime: attributes.mime,
            url: attributes.url,
            width: attributes.width,
            height: attributes.height,
            name: attributes.name,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Supporter"
)]
pub struct Supporter {
    pub id: Option<String>,
    pub contact_information: ContactInformation,
    pub portrait: Option<Image>,
    pub virtual_cats: Vec<Cat>,
}

impl TryFrom<SourceSupporter> for Supporter {
    type Error = Error;
    fn try_from(value: SourceSupporter) -> Result<Supporter, Error> {
        let SourceSupporter {
            contact_information,
            portrait,
            virtual_cats,
        } = value;
        let cats: Result<Vec<Cat>, Error> = virtual_cats
            .context(MissingAttributeSnafu {})?
            .data
            .into_iter()
            .map(|virtual_cat_entity| {
                virtual_cat_entity
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

        let portrait: Result<Option<Image>, Error> = portrait
            .context(MissingAttributeSnafu {})?
            .data
            .map(|image_entity| {
                image_entity
                    .attributes
                    .context(MissingAttributeSnafu {})?
                    .image
                    .try_into()
            })
            .transpose();

        let contact_information: ContactInformation = contact_information
            .context(MissingAttributeSnafu {})?
            .data
            .context(MissingAttributeSnafu {})?
            .attributes
            .context(MissingAttributeSnafu {})?;

        Ok(Supporter {
            virtual_cats: cats?,
            portrait: portrait?,
            contact_information,
            id: None,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.LookingForHomeCat"
)]
pub struct LookingForHomeCat {
    pub id: Option<String>,
    pub caretaker: Option<ContactInformation>,
    pub cat: Cat,
}

impl TryFrom<LookingForAdoptionCat> for LookingForHomeCat {
    type Error = Error;
    fn try_from(value: LookingForAdoptionCat) -> Result<LookingForHomeCat, Error> {
        let LookingForAdoptionCat { caretaker, cat } = value;
        let inner_cat: Cat = cat
            .context(MissingAttributeSnafu {})?
            .data
            .context(MissingAttributeSnafu {})?
            .attributes
            .context(MissingAttributeSnafu {})?
            .into();

        let caretaker: Option<ContactInformation> = caretaker
            .context(MissingAttributeSnafu {})?
            .data
            .and_then(|caretaker_entity| caretaker_entity.attributes);

        Ok(LookingForHomeCat {
            id: None,
            cat: inner_cat,
            caretaker,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.AdoptedCat"
)]
pub struct AdoptedCat {
    pub id: Option<String>,
    pub adoption_date: String,
    pub caretaker: Option<ContactInformation>,
    pub cat: Cat,
}
impl TryFrom<SourceAdoptedCat> for AdoptedCat {
    type Error = Error;
    fn try_from(value: SourceAdoptedCat) -> Result<AdoptedCat, Error> {
        let SourceAdoptedCat {
            adoption_date,
            cat,
            caretaker,
        } = value;
        let DateTime(inner_datetime_string) = adoption_date;
        let inner_cat: Cat = cat
            .context(MissingAttributeSnafu {})?
            .data
            .context(MissingAttributeSnafu {})?
            .attributes
            .context(MissingAttributeSnafu {})?
            .into();
        let caretaker: Option<ContactInformation> = caretaker
            .context(MissingAttributeSnafu {})?
            .data
            .and_then(|caretaker_entity| caretaker_entity.attributes);

        Ok(AdoptedCat {
            id: None,
            caretaker,
            adoption_date: inner_datetime_string,
            cat: inner_cat,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.LostCat"
)]
pub struct LostCat {
    pub id: Option<String>,
    pub disappearance_circumstances: String,
    pub disappearance_location: String,
    pub disappearance_datetime: String,
    pub during_medical_treatment: bool,
    pub special_signs: Option<String>,
    pub cat: Cat,
}

impl TryFrom<SourceLostCat> for LostCat {
    type Error = Error;
    fn try_from(value: SourceLostCat) -> Result<LostCat, Error> {
        let SourceLostCat {
            disappearance_circumstances,
            disappearance_datetime,
            disappearance_location,
            during_medical_treatment,
            special_signs,
            cat,
        } = value;
        let DateTime(disappearance_datetime) = disappearance_datetime;
        let cat: Cat = cat
            .context(MissingAttributeSnafu {})?
            .data
            .context(MissingAttributeSnafu {})?
            .attributes
            .context(MissingAttributeSnafu {})?
            .into();

        Ok(LostCat {
            id: None,
            disappearance_circumstances,
            disappearance_datetime,
            disappearance_location,
            during_medical_treatment,
            special_signs,
            cat,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.FoundCat"
)]
pub struct FoundCat {
    pub id: Option<String>,
    pub discovery_circumstances: String,
    pub found_location: String,
    pub found_datetime: String,
    pub special_signs: Option<String>,
    pub cat: Cat,
}

impl TryFrom<SourceFoundCat> for FoundCat {
    type Error = Error;
    fn try_from(value: SourceFoundCat) -> Result<FoundCat, Error> {
        let SourceFoundCat {
            discovery_circumstances,
            found_datetime,
            found_location,
            special_signs,
            cat,
        } = value;
        let DateTime(found_datetime) = found_datetime;
        let cat: Cat = cat
            .context(MissingAttributeSnafu {})?
            .data
            .context(MissingAttributeSnafu {})?
            .attributes
            .context(MissingAttributeSnafu {})?
            .into();
        Ok(FoundCat {
            id: None,
            discovery_circumstances,
            found_datetime,
            found_location,
            special_signs,
            cat,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Article"
)]
pub struct Article {
    pub id: Option<String>,
    pub title: String,
    pub image: Option<Image>,
    pub introduction: String,
    pub content: String,
}

impl TryFrom<SourceArticleAnnouncement> for Article {
    type Error = Error;
    fn try_from(value: SourceArticleAnnouncement) -> Result<Article, Error> {
        let SourceArticleAnnouncement { article, title } = value;
        let ArticleEntityResponse { data } = article.context(MissingAttributeSnafu {})?;
        let ArticleEntity { attributes, id } = data.context(MissingAttributeSnafu {})?;
        let SourceArticle {
            image,
            content,
            introduction,
        } = attributes.context(MissingAttributeSnafu {})?;
        let image: Option<Image> = image.try_into().ok();
        let id: Option<String> = id.map(|id| id.into_inner());
        Ok(Article {
            id,
            title,
            image,
            content,
            introduction,
        })
    }
}
#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.ExternalMedia"
)]
pub struct ExternalMedia {
    pub id: Option<String>,
    pub title: String,
    pub media_url: String,
    pub tags: Vec<String>,
    pub image: Option<Image>,
}

impl From<SourceExternalMedia> for ExternalMedia {
    fn from(value: SourceExternalMedia) -> Self {
        let SourceExternalMedia {
            image,
            tags,
            title,
            media_url,
        } = value;
        let image: Option<Image> = image.try_into().ok();
        let tags: Vec<String> = tags.map_or_else(Vec::new, |tag_collection| {
            tag_collection
                .data
                .into_iter()
                .filter_map(|tag_entity| tag_entity.attributes.map(|tag| tag.text))
                .collect()
        });

        ExternalMedia {
            id: None,
            title,
            media_url,
            tags,
            image,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Announcement"
)]
pub struct Announcement {
    pub id: Option<String>,
    pub title: String,
    pub tags: Vec<String>,
    pub image: Option<Image>,
}

impl From<SourceAnnouncement> for Announcement {
    fn from(value: SourceAnnouncement) -> Self {
        let SourceAnnouncement {
            title,
            image,
            announcement_tags,
            ..
        } = value;
        let image: Option<Image> = image.try_into().ok();
        let tags: Vec<String> = announcement_tags.map_or_else(Vec::new, |tag_collection| {
            tag_collection
                .data
                .into_iter()
                .filter_map(|tag_entity| tag_entity.attributes.map(|tag| tag.text))
                .collect()
        });

        Announcement {
            id: None,
            title,
            tags,
            image,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Cat"
)]
pub struct Cat {
    pub id: Option<String>,
    pub age: Option<Age>,
    pub chip_number: Option<String>,
    pub name: Option<String>,
    pub slug: String,
    pub sex: Sex,
    pub medical_status: Option<MedicalStatus>,
    pub fiv_felv: Option<FivFelv>,
    pub healthy: Option<bool>,
    pub tags: Vec<String>,
    pub description_heading: Option<String>,
    pub description: Option<String>,
    pub is_dead: bool,
    pub castrated: Option<bool>,
    pub color: Color,
    pub contact_informations: Vec<ContactInformation>,
    pub images: Vec<Image>,
}

impl From<SourceCat> for Cat {
    fn from(value: SourceCat) -> Self {
        let SourceCat {
            name,
            chip_number,
            contact_informations,
            slug,
            sex,
            medical_status,
            fiv_felv,
            healthy,
            cat_tags,
            description_heading,
            description,
            is_dead,
            castrated,
            color,
            age,
            ..
        } = value;

        let contact_informations: Vec<ContactInformation> =
            contact_informations.map_or_else(Vec::new, |contact_collection| {
                contact_collection
                    .data
                    .into_iter()
                    .filter_map(|contact_entity| contact_entity.attributes)
                    .collect()
            });

        let tags: Vec<String> = cat_tags.map_or_else(Vec::new, |tag_collection| {
            tag_collection
                .data
                .into_iter()
                .filter_map(|tag_entity| tag_entity.attributes.map(|tag| tag.text))
                .collect()
        });

        let images: Vec<Image> = value.images.map_or_else(Vec::new, |images| {
            images
                .data
                .into_iter()
                .flat_map(|image_entity| {
                    image_entity.attributes.map(|image| {
                        image.image.data.map(|upload_entity| {
                            let id = upload_entity.id.map(|id| id.into_inner());
                            upload_entity.attributes.map(|upload| {
                                let UploadFile {
                                    name,
                                    url,
                                    mime,
                                    width,
                                    height,
                                    preview_url,
                                    alternative_text,
                                } = upload;
                                Image {
                                    id,
                                    name,
                                    url,
                                    mime,
                                    width,
                                    height,
                                    preview_url,
                                    alternative_text,
                                }
                            })
                        })
                    })
                })
                .flatten()
                .flatten()
                .collect()
        });

        Cat {
            id: None,
            age,
            contact_informations,
            chip_number,
            name,
            slug,
            sex,
            medical_status,
            fiv_felv,
            healthy,
            description_heading,
            description,
            is_dead,
            castrated,
            color,
            tags,
            images,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Paged"
)]
pub struct Paged<
    #[cfg(not(feature = "elixir_support"))] T,
    #[cfg(feature = "elixir_support")] T: rustler::Encoder,
> {
    pub items: Vec<T>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub page_count: i32,
}

impl<
        #[cfg(not(feature = "elixir_support"))] T,
        #[cfg(feature = "elixir_support")] T: rustler::Encoder,
    > Paged<T>
{
    pub fn new(pagination: Pagination, items: Vec<T>) -> Paged<T> {
        Paged {
            items,
            total: pagination.total,
            page: pagination.page,
            page_size: pagination.page_size,
            page_count: pagination.page_count,
        }
    }
}
