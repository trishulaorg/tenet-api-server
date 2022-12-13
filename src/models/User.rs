use async_graphql::*;
use sea_orm::entity::prelude::*;

use serde::{Serialize, Deserialize};
use chrono::{Utc, Local, DateTime, Date};

#[derive(SimpleObject)]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: ID,
  pub token: String,
}
