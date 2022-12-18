use async_graphql::*;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use entities::*;

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub token: String,
}

impl From<entities::user::Model> for User {
  fn from(model: entities::user::Model) -> Self {
    Self {
      id: model.id,
      created_at: model.created_at,
      token: model.token
    }
  }
}

#[derive(Clone, Debug, SimpleObject)]
pub struct Persona {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub screen_name: String,
    pub icon_url: String,
    pub user_id: i32,
}