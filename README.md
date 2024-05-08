## Example on creating a new query
### Generate Cynic
- Visit [cynic rust generator](https://generator.cynic-rs.dev/) 
- Load `https://kotkowo-admin.ravensiris.xyz/graphql`
- Headers: `{"Authorization": "Bearer <TOKEN>"}`
- Select query on the left

### Create Cynic query
- Paste Cynic generated query in `src/queries/my_query.rs`
- Remove duplicate structs featured in `src/queries/commons.rs`
- Remove lifetime `&str` and such

### Create impl and struct in models.rs
- example
```rust
use crate::queries::cat::Announcement as SourceAnnouncement;

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
            id: None, // we will skip the id for now
            title,
            image,
        }
    }
}
```

### Create a function in lib.rs
- example
```rust
pub fn list_announcement(
    options: Options<AnnouncementFiltersInput>,
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
        filters: None, // The Announcements won't need filtering so we omit it here
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
```
### Create a test for the new func
- example
```rust
impl Default for AnnouncementFiltersInput {
    fn default() -> Self {
        AnnouncementFiltersInput {}
    }
}

#[cfg(test)]
mod tests {
    use crate::{list_announcement, Options};

    #[test]
    fn list_announcement_test() {
        let paged = list_announcement(Options::default());
        assert!(paged.is_ok());
    }
}
```
