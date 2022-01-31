use std::io::Error;
use mkpm_sl::{Book, List, ReadJson, Search};
use rocket::get;
use rocket::serde::json::Json;

#[get("/read/<title>")]
pub fn get_read(title: &str) -> Result<Json<Book>, Error>{
    let books = Book::read_json("json/books.json")?;
    let b = Book::search(title, &books).unwrap();
    Ok(Json(b))
}

#[get("/read/list")]
pub fn get_read_list() -> Result<Json<Vec<Book>>, Error>{
    Ok(Json(Book::read_json("json/books.json")?))
}