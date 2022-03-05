use agql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql as agql;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use model::{mutation::MutationRoot, query::QueryRoot, WikitSchema};
use rocket::{response::content, State};
use sqlx::postgres::PgPoolOptions;

mod db;
mod model;

#[rocket::get("/")]
fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<WikitSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<WikitSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

#[rocket::launch]
async fn rocket() -> _ {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres:///wikit_dev")
        .await
        .unwrap();
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();
    rocket::build().manage(schema).mount(
        "/",
        rocket::routes![graphql_playground, graphql_query, graphql_request],
    )
}
