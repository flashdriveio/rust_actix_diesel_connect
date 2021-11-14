use actix_web::{ web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
// mod errors;
// mod models;
// mod schema;
mod postgres;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let pool = postgres::get_pool();
 
    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
            .route("/", web::get().to(controllers::get_all_books))
            .route("/create-books", web::post().to(controllers::create_new_book))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
