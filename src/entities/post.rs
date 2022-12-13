//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use super::sea_orm_active_enums::ContentType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "deletedAt")]
    pub deleted_at: Option<DateTime>,
    pub title: String,
    #[sea_orm(column_name = "contentType")]
    pub content_type: ContentType,
    pub content: String,
    #[sea_orm(column_name = "boardId")]
    pub board_id: String,
    #[sea_orm(column_name = "personaId")]
    pub persona_id: i32,
    #[sea_orm(column_name = "defaultPostRoleId")]
    pub default_post_role_id: i32,
    #[sea_orm(column_name = "defaultThreadRoleId")]
    pub default_thread_role_id: i32,
    #[sea_orm(column_name = "defaultReplyRoleId")]
    pub default_reply_role_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
