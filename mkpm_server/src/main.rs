mod clone;
mod app;
mod install;
mod list;
mod read;

use clone::{get_clone, get_clone_list};
use app::{get_app, get_app_list};
use read::{get_read, get_read_list};
use install::{get_install, get_install_list};
use rocket::{get, launch, post, routes, Response};
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_clone, get_app, get_read, get_install, get_read_list, get_clone_list, get_install_list, get_app_list])
}
