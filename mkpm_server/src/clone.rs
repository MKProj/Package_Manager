use mkpm_sl::{Clone, ReadJson, Search};
use rocket::get;
use rocket::serde::json::Json;
use std::io::Error;

#[get("/clone/<repo>")]
pub fn get_clone(repo: &str) -> Result<Json<Clone>, Error> {
    let vec_repo = Clone::read_json("json/repo.json")?;
    let repo = Clone::search(repo, &vec_repo).unwrap();
    Ok(Json(repo))
}
