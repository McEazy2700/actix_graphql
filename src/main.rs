mod gql;
mod schema;
use actix_web::{web, App, HttpServer, HttpResponse, web::Data};
use async_graphql::{Schema, EmptyMutation, EmptySubscription, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::Query;



#[actix_web::post("/")]
async fn graphql(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}


#[actix_web::get("/")]
async fn graphiql() -> std::io::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish();

    println!("Running server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(graphql)
            .service(graphiql)
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
