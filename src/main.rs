#![deny(missing_docs)]
/*!
* Backend for Banur
*/

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/**
 * Defines the main server for the backend
 */

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(index))
  })
  .bind("localhost:8080")?
  .run()
  .await
}

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}
