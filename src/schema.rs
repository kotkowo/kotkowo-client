#[cynic::schema("kotkowo")]
mod schema {}
pub use schema::*;

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

impl Paged<crate::queries::cat::Cat> {
    pub fn new(
        pagination: crate::queries::cat::Pagination,
        items: Vec<crate::queries::cat::Cat>,
    ) -> Paged<crate::queries::cat::Cat> {
        Paged {
            items,
            total: pagination.total,
            page: pagination.page,
            page_size: pagination.page_size,
            page_count: pagination.page_count,
        }
    }
}
