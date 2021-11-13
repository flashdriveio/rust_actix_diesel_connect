#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod controllers;
// mod errors;
// mod models;
// mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database = std::env::var("DATABASE_URL").expect("Missing Database URI");

    let manager = ConnectionManager::<PgConnection>::new(database);

    let pool: Pool = r2d2::Pool::Builder().build(manager).expect("Failed database connection");
    
    HttpServer::new(move || {
        App::new()
            .route("/books", web::get().to(controllers::get_all_books))
            .route("/books", web::post().to(controllers::create_new_book))
    })
    .bind("127.0.0.1:5003")?
    .run()
    .await
}
