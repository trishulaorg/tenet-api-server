use async_graphql::{Object};
use async_graphql::{Context};
use sea_orm::{DatabaseConnection, EntityTrait, DbErr};
use crate::models::User;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, context: &Context<'_>, id: i32) -> Result<Option<User>, DbErr> {
      let db = context.data::<DatabaseConnection>().unwrap();
      let user = entities::user::Entity::find_by_id(id).one(db).await?;
      Ok(user.map(User::from))
    }
}
