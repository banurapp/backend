#![deny(missing_docs)]
/*!
* Backend for Banur
*/

use std::sync::Arc;
use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

/// Contains the GraphQL schema
mod schema;
use crate::schema::{create_schema, Schema};

/**
 * Defines the main server for the backend
 */

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
      App::new()
        .data(schema.clone())
        .route("/", web::get().to(index))
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
        .bind("localhost:8080")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn graphiql() -> HttpResponse {
  let html = graphiql_source("http://127.0.0.0:8080/graphql");
  HttpResponse::Ok().body(html)
}

async fn graphql(st: web::Data<Arc<Schema>>, data: web::Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
  let user = web::block(move || {
    let res = data.execute(&st, &schema::Context {} );
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .await?;
  Ok(HttpResponse::Ok().content_type("application/json").body(user))
}
