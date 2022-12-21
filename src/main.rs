use ::graphql::{queries::*, Token};

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_poem::{GraphQL, GraphQLRequest, GraphQLResponse};
use poem::{get, handler, listener::TcpListener, web::{Html, Data}, IntoResponse, Route, Server, http::HeaderMap};

use tenet_api_server::db_connection;
use dotenv;
use std::env;

#[handler]
async fn graphql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .finish(),
    )
}


fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

#[handler]
async fn index(
    schema: Data<&Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;
    if let Some(token) = get_token_from_headers(headers) {
        req = req.data(token);
    }
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let database_url: std::string::String = env::var("DATABASE_URL").unwrap();
    let database_name: std::string::String = env::var("DATABASE_NAME").unwrap();
    // create the schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db_connection(&database_url, &database_name).await)
        .finish();

    // start the http server
    let app = Route::new().at("/", get(graphql).post(index));
    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}
