//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "SystemAdministratorRole")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "allowAll")]
    pub allow_all: i8,
    #[sea_orm(column_name = "roleManager")]
    pub role_manager: i8,
    #[sea_orm(column_name = "boardRoleId")]
    pub board_role_id: i32,
    #[sea_orm(column_name = "postRoleId")]
    pub post_role_id: i32,
    #[sea_orm(column_name = "threadRoleId")]
    pub thread_role_id: i32,
    #[sea_orm(column_name = "replyRoleId")]
    pub reply_role_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
