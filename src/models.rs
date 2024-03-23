pub use crate::queries::cat::{Age, Color, FivFelv, MedicalStatus, Pagination, Sex};
use crate::queries::cat::{Cat as SourceCat, UploadFile};

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

#[derive(Debug)]
#[cfg_attr(
    feature = "elixir_support",
    derive(rustler::NifStruct),
    module = "Kotkowo.Client.Cat"
)]
pub struct Cat {
    pub id: Option<String>,
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
            ..
        } = value;

        let tags: Vec<String> = cat_tags.map_or_else(
            || vec![],
            |tag_collection| {
                tag_collection
                    .data
                    .into_iter()
                    .filter_map(|tag_entity| tag_entity.attributes.map(|tag| tag.text))
                    .collect()
            },
        );

        let images: Vec<Image> = value.images.map_or_else(
            || vec![],
            |images| {
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
            },
        );

        Cat {
            id: None,
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
    #[cfg(feature = "elixir_support")] T: rustler::Encoder + for<'a> rustler::Decoder<'a>,
> {
    pub items: Vec<T>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub page_count: i32,
}

impl Paged<Cat> {
    pub fn new(pagination: Pagination, items: Vec<Cat>) -> Paged<Cat> {
        Paged {
            items,
            total: pagination.total,
            page: pagination.page,
            page_size: pagination.page_size,
            page_count: pagination.page_count,
        }
    }
}
