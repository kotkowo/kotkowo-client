use std::env::VarError;

use crate::queries::{
    announcement::Announcement as SourceAnnouncement,
    cat::Cat as SourceCat,
    commons::{UploadFile, UploadFileEntityResponse},
};
pub use crate::queries::{
    cat::{Age, Color, FivFelv, MedicalStatus, Sex},
    commons::Pagination,
};

use cynic::http::CynicReqwestError;
use reqwest::header::InvalidHeaderValue;
use snafu::OptionExt;

use snafu::{Backtrace, Snafu};

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
    module = "Kotkowo.Client.Announcement"
)]
pub struct Announcement {
    pub id: Option<String>,
    pub title: String,
    pub image: Option<Image>,
}

impl From<SourceAnnouncement> for Announcement {
    fn from(value: SourceAnnouncement) -> Self {
        let SourceAnnouncement { title, image, .. } = value;
        let image: Option<Image> = image.try_into().ok();
        Announcement {
            id: None,
            title,
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
    pub age: Age,
    pub name: String,
    pub slug: String,
    pub sex: Sex,
    pub medical_status: MedicalStatus,
    pub fiv_felv: FivFelv,
    pub healthy: bool,
    pub tags: Vec<String>,
    pub description_heading: String,
    pub description: String,
    pub is_dead: bool,
    pub castrated: bool,
    pub color: Color,
    pub images: Vec<Image>,
}

impl From<SourceCat> for Cat {
    fn from(value: SourceCat) -> Self {
        let SourceCat {
            name,
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
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Missing or none attribute"))]
    MissingAttribute { backtrace: Backtrace },

    #[snafu(display("Request failure"))]
    CynicRequestError {
        source: CynicReqwestError,
        backtrace: Backtrace,
    },

    #[snafu(display("Request failure"))]
    RequestError {
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("{:?}", message))]
    RequestResultedInError { message: String },

    #[snafu(display("Environment variable missing"))]
    EnvVarMissing {
        source: VarError,
        backtrace: Backtrace,
    },

    #[snafu(display("Invalid header value"))]
    InvalidHeaderValue {
        source: InvalidHeaderValue,
        backtrace: Backtrace,
    },
}
