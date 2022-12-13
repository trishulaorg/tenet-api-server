//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5
use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "AllowedWritingRole")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub create: i8,
    pub read: i8,
    pub update: i8,
    pub delete: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
