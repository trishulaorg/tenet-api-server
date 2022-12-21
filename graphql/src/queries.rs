use async_graphql::{Object};
use async_graphql::{Context};
use jsonwebtoken::{DecodingKey, Validation, errors::Error};
use sea_orm::{DatabaseConnection, EntityTrait, DbErr, QueryFilter, ColumnTrait};
use crate::{Token, Claims};
use crate::models::{User, Post};
use anyhow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, context: &Context<'_>, id: i32) -> anyhow::Result<Option<User>> {
      let db = context.data::<DatabaseConnection>().unwrap();
      let user = entities::user::Entity::find_by_id(id).one(db).await?;
      Ok(user.map(User::from))
    }
    async fn activities(&self, context: &Context<'_>) -> anyhow::Result<Vec<Post>> {
      let db = context.data::<DatabaseConnection>().unwrap();
      let posts = entities::post::Entity::find().all(db).await?;
      Ok(posts.into_iter().map(Post::from).collect())
    }
    async fn me(&self, context: &Context<'_>) -> anyhow::Result<Option<User>> {
      let db = context.data::<DatabaseConnection>().unwrap();
      let token = context.data::<Token>().unwrap();
      // decode jwt 
      let token = jsonwebtoken::decode::<Claims>(&token.0.to_owned(), &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
      // token.claims.

      let me = entities::user::Entity::find().filter(entities::user::Column::Id.eq(3)).one(db).await?;
      Ok(me.map(User::from))
    }
}
