use async_graphql::*;

#[derive(Clone, Debug, SimpleObject)]
pub struct User {
    pub id: ID,
    // pub created_at: DateTime,
    pub token: String,
}