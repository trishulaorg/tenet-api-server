use async_graphql::Object;

use crate::models::User;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hi(&self) -> User {
        User {
            id: "id".into(),
            token: "hi".into(),
        }
    }
}
