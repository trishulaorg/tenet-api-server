//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "Bot")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "personaId", unique)]
    pub persona_id: i32,
    #[sea_orm(column_name = "thirdPartyAPIKeyId", unique)]
    pub third_party_api_key_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
