//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5
use super::sea_orm_active_enums::ContentType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "deletedAt")]
    pub deleted_at: Option<DateTime>,
    pub content: String,
    #[sea_orm(column_name = "contentType")]
    pub content_type: ContentType,
    #[sea_orm(column_name = "boardId")]
    pub board_id: String,
    #[sea_orm(column_name = "postId")]
    pub post_id: String,
    #[sea_orm(column_name = "personaId")]
    pub persona_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
