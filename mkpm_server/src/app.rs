use mkpm_sl::{Application, ReadJson, Search};
use rocket::get;
use std::io::Error;
use rocket::serde::json::Json;

#[get("/get/<name>")]
pub fn get_app(name: &str) -> Result<Json<Application>, Error> {
    let v = Application::read_json("json/application.json")?;
    let a = Application::search(name, &v).unwrap();
    Ok(Json(a))
}

#[get("/get/list")]
pub fn get_app_list() -> Result<Json<Vec<Application>>, Error>{
    Ok(Json(Application::read_json("json/application.json")?))
}