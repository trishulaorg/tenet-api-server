use std::{sync::Arc, time::Duration};
use chrono::{Utc, DateTime};
use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};

#[derive(Clone)]
pub struct activity {
  id: ID,
  createdAt: DateTime<Utc>,
  title: String,
  content: String,
}