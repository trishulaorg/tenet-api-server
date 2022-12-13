//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5
use crate::graphql;

use super::{reply, sea_orm_active_enums::ContentType};
use async_graphql::{ComplexObject, Context, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(complex, name = "Thread")]
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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
    #[sea_orm(has_many = "super::reply::Entity")]
    Reply,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::reply::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reply.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl Model {
    async fn post(&self, ctx: &Context<'_>) -> Result<Option<super::post::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(crate::Post).one(db).await
    }
    async fn replies(&self, ctx: &Context<'_>) -> Result<Vec<reply::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(crate::Reply).all(db).await
    }
}
