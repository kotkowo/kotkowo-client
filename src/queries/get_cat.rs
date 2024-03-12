use crate::schema;

#[derive(cynic::QueryVariables, Debug)]
pub struct GetCatVariables<'a> {
    pub id: &'a cynic::Id,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GetCatVariables")]
pub struct GetCat {
    #[arguments(id: $id)]
    pub cat: Option<CatEntityResponse>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CatEntityResponse {
    pub data: Option<CatEntity>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CatEntity {
    pub attributes: Option<Cat>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Cat {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cat_get_query() {
        use cynic::QueryBuilder;
        let id = &cynic::Id::new("test");
        let operation = GetCat::build(
            GetCatVariables{
                id,
        });
        insta::assert_snapshot!(operation.query);
    }
}
