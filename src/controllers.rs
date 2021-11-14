use actix_web::Responder;
use super::models::{NewBook, Book};
use super::schema::books::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{ insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputBook {
    pub name: String,
    pub author: String,
    pub year: String,
}

pub async fn get_all_books()-> impl Responder{
    format!("Hello books")

}



pub async fn create_new_book(
    db: web::Data<Pool>,
    item: web::Json<InputBook>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_book(db, item))
        .await
        .map(|book| HttpResponse::Created().json(book))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_book(
    db: web::Data<Pool>,
    item: web::Json<InputBook>,
) -> Result<Book, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_book = NewBook {
        name: &item.name,
        author: &item.author,
        year: &item.year,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(books).values(&new_book).get_result(&conn)?;
    Ok(res)
}
