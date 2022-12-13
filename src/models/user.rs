use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};
use chrono::{Utc, Local, DateTime, Date};

#[derive(SimpleObject)]
pub struct User {
  id: ID,
  token: String,
}
