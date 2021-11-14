use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub year: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub name: &'a str,
    pub author: &'a str,
    pub year: &'a str,
    pub created_at: chrono::NaiveDateTime,
}