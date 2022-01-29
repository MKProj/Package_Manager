use mkpm_sl::{Application, ReadJson, Search};
use rocket::get;
use std::io::Error;
use rocket::serde::json::Json;

#[get("/app/<name>")]
pub fn get_app(name: &str) -> Result<Json<Application>, Error> {
    let v = Application::read_json("json/application.json")?;
    let a = Application::search(name, &v).unwrap();
    Ok(Json(a))
}
