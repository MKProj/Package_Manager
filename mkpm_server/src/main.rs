mod clone;
mod app;
mod install;
mod list;
mod read;

use clone::get_clone;
use app::get_app;
use read::get_read;
use install::get_install;
use rocket::{get, launch, post, routes, Response};
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_clone, get_app, get_read, get_install])
}
