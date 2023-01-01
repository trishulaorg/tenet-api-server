use async_graphql::*;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use std::string::String;

// Model: User
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub created_at: NaiveDateTime,
}

#[ComplexObject]
impl User {
    async fn personas(&self, context: &Context<'_>) -> Result<Vec<Persona>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let personas = entities::persona::Entity::find()
            .filter(entities::persona::Column::UserId.eq(self.id.clone()))
            .all(db)
            .await?;
        Ok(personas.into_iter().map(Persona::from).collect())
    }
}

impl From<entities::user::Model> for User {
    fn from(model: entities::user::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
        }
    }
}
// Model: Persona
#[derive(Clone, Debug, SimpleObject)]
pub struct Persona {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub screen_name: String,
    pub icon_url: String,
    pub user_id: String,
}

impl From<entities::persona::Model> for Persona {
    fn from(model: entities::persona::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            name: model.name,
            screen_name: model.screen_name,
            icon_url: model.icon_url,
            user_id: model.user_id,
        }
    }
}

// Model: Persona
#[derive(Clone, Debug, SimpleObject)]
pub struct Bot {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub token: String,
    pub name: String,
    pub screen_name: String,
    pub icon_url: String,
    pub user_id: String,
}

impl From<entities::bot::Model> for Bot {
    fn from(model: entities::bot::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            name: model.name,
            token: model.token,
            screen_name: model.screen_name,
            icon_url: model.icon_url,
            user_id: model.user_id,
        }
    }
}
// Model: Post
#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub title: String,
    pub content: String,
    pub board_id: String,
    pub persona_id: String,
}

impl From<entities::post::Model> for Post {
    fn from(model: entities::post::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            title: model.title,
            content: model.content,
            board_id: model.board_id,
            persona_id: model.persona_id,
        }
    }
}

#[ComplexObject]
impl Post {
    async fn threads(&self, context: &Context<'_>) -> Result<Vec<Thread>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let threads = entities::thread::Entity::find()
            .filter(entities::thread::Column::PostId.eq(self.id.clone()))
            .all(db)
            .await?;
        Ok(threads.into_iter().map(Thread::from).collect())
    }
}
// Model: Thread
#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct Thread {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub content: String,
    pub post_id: String,
    pub persona_id: String,
}

impl From<entities::thread::Model> for Thread {
    fn from(model: entities::thread::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            content: model.content,
            post_id: model.post_id,
            persona_id: model.persona_id,
        }
    }
}

#[ComplexObject]
impl Thread {
    async fn replies(&self, context: &Context<'_>) -> Result<Vec<Reply>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let replies = entities::reply::Entity::find()
            .filter(entities::reply::Column::ThreadId.eq(self.id.clone()))
            .all(db)
            .await?;
        Ok(replies.into_iter().map(Reply::from).collect())
    }
}

// Model: Reply
#[derive(Clone, Debug, SimpleObject)]
pub struct Reply {
    pub id: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub thread_id: String,
    pub persona_id: String,
}
impl From<entities::reply::Model> for Reply {
    fn from(model: entities::reply::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            content: model.content,
            thread_id: model.thread_id,
            persona_id: model.persona_id,
        }
    }
}

// pub struct VoteOnReply {
//   pub id: String,
//   pub createdAt: NaiveDateTime,
//   pub createdById: String,
//   pub weight: Int
// }
