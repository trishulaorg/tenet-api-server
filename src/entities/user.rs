//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5
use crate::persona;
use async_graphql::{ComplexObject, Context, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(complex, name = "User")]
#[sea_orm(table_name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(unique)]
    pub token: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::persona::Entity")]
    Persona,
}

impl Related<super::persona::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Persona.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl Model {
    async fn personas(&self, ctx: &Context<'_>) -> Result<Vec<persona::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(crate::Persona).all(db).await
    }
}
