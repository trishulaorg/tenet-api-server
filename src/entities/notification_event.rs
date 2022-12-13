//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5
use super::sea_orm_active_enums::Type;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "NotificationEvent")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "senderId")]
    pub sender_id: i32,
    #[sea_orm(column_name = "receiverId")]
    pub receiver_id: i32,
    pub r#type: Type,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "readAt")]
    pub read_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
