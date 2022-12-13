
use dotenv::dotenv;
use sea_orm::{DatabaseConnection, Database};
use std::env;


use tracing_subscriber;
use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema, parser::Error,
};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

mod model;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn howdy<'a>(
        &self,
        _ctx: &Context<'a>,
        #[graphql(desc = "id of the human")] _id: String,
    ) -> &str {
        "howdy"
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

async fn database() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").unwrap();
    Database::connect(db_url).await.unwrap()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    // create the schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    dotenv().ok();
    
    let db = database();


    // start the http server
    let app = Route::new().at("/", get(graphql).post(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}
