use rocket::get;
use mkpm_sl::{CargoApp, ReadJson, Search};
use std::io::Error;
use rocket::serde::json::Json;

#[get("/install/<name>")]
pub fn get_install(name: &str) -> Result<Json<CargoApp>, Error>{
    let apps = CargoApp::read_json("json/cargo.json")?;
    let app = CargoApp::search(name, &apps).unwrap();
    Ok(Json(app))
}
