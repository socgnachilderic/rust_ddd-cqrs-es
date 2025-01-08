use async_graphql::*;
use post_resolvers::{PostMutation, PostQuery};

mod post_resolvers;

#[derive(Default, MergedObject)]
pub struct Query(QueryRoot, PostQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(PostMutation);

#[derive(Default)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn health_check(&self) -> &str {
        "OK"
    }
}
