use crate::models::{Board};
use async_graphql::*;
use chrono::Utc;
use sea_orm::{DatabaseConnection, Set};
use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Mutation;

#[Object]
impl Mutation {
  async fn create_board(&self, context: &Context<'_>, title: String, content: String)  -> anyhow::Result<Option<Board>> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let board = entities::board::ActiveModel {
          title: Set(title),
          content: Set(content.clone()),
          raw_content: Set(content.clone()),
          created_at: Set(Utc::now().naive_utc()),
          id: Set(Ulid::new().to_string())
        };
       Ok(board.into())
  }
}