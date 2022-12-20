use async_graphql::*;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use entities::*;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(complex)] 
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub token: String,
}

#[ComplexObject]
impl User {
    async fn personas(&self, context: &Context<'_>) -> Result<Vec<Persona>, DbErr> {
      let db = context.data::<DatabaseConnection>().unwrap();
      let personas = entities::persona::Entity::find().filter(entities::persona::Column::UserId.eq(self.id)).all(db).await?;
      Ok(personas.into_iter().map(Persona::from).collect())
  }
}

impl From<entities::user::Model> for User {
  fn from(model: entities::user::Model) -> Self {
    Self {
      id: model.id,
      created_at: model.created_at,
      token: model.token,
    }
  }
}

#[derive(Clone, Debug, SimpleObject)]
pub struct Persona {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub screen_name: String,
    pub icon_url: String,
    pub user_id: i32,
}

impl From<entities::persona::Model> for Persona {
  fn from(model: entities::persona::Model) -> Self {
    Self {
      id: model.id,
      created_at: model.created_at,
      name: model.name,
      screen_name: model.screen_name,
      icon_url: model.icon_url,
      user_id: model.user_id,
    }
  }
}