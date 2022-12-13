use futures::executor::block_on;

use dotenv::dotenv;
use sea_orm::{Database, DbErr};

use std::env;


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

async fn run() -> Result<(), DbErr> {
    let DATABASE_URL = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
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
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // create the schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    // start the http server
    let app = Route::new().at("/", get(graphql).post(GraphQL::new(schema)));

    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}
