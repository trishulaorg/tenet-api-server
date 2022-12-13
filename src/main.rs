mod setup;

use futures::executor::block_on;

use sea_orm::{Database, DbErr, DatabaseConnection, ConnectionTrait};

use std::env;

use setup::set_up_db;

use tracing_subscriber;
use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema, parser::Error,
};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn howdy<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the human")] _id: String,
    ) -> &str {
        "aaa"
    
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
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(db).finish();

    // start the http server
    let app = Route::new().at("/", get(graphql).post(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}
