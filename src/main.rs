mod setup;



use sea_orm::{DatabaseConnection, DbErr};



use setup::set_up_db;

use crate::entities::{prelude::*, *};

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription,
    Object, Schema,
};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};


mod entities;

use sea_orm::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn howdy<'a>(
        &self,
        _ctx: &Context<'a>,
        #[graphql(desc = "id of the human")] _id: String,
    ) -> String {
        "aaa".to_owned()
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        User::find_by_id(id).one(db).await
    }
    async fn users(&self, ctx: &Context<'_>, _id: i32) -> Result<Vec<user::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        User::find().all(db).await
    }
    async fn persona(&self, ctx: &Context<'_>, id: i32) -> Result<Option<persona::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Persona::find_by_id(id).one(db).await
    }
    async fn personas(&self, ctx: &Context<'_>, _id: i32) -> Result<Vec<persona::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Persona::find().all(db).await
    }
    async fn activities(&self, ctx: &Context<'_>) -> Result<Vec<post::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Post::find().all(db).await
    }
    async fn post(&self, ctx: &Context<'_>, id: String) -> Result<Option<post::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Post::find_by_id(id).one(db).await
    }
    async fn thread(&self, ctx: &Context<'_>, id: String) -> Result<Option<thread::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Thread::find_by_id(id).one(db).await
    }
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<post::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Post::find().all(db).await
    }
    async fn threads(&self, ctx: &Context<'_>) -> Result<Vec<thread::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Thread::find().all(db).await
    }
}

#[handler]
async fn graphql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // init db
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    // create the schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    // start the http server
    let app = Route::new().at("/", get(graphql).post(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}
