//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Persona")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(column_name = "screenName")]
    pub screen_name: String,
    #[sea_orm(column_name = "iconUrl", column_type = "Text")]
    pub icon_url: String,
    #[sea_orm(column_name = "userId")]
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
       belongs_to = "super::user::Entity",
       from = "Column::UserId",
       to = "super::user::Column::Id"
    )]
    User,
}


impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
