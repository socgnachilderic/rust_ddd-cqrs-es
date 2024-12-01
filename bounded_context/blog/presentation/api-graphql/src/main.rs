use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    http::Method,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use blog_api::InjectionContainer;
use blog_api_graphql::{Mutation, Query};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let inject_api = InjectionContainer::new().await;
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(inject_api)
        .enable_federation()
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST]),
        );

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
