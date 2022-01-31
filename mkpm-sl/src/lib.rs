pub mod application;
pub mod book;
pub mod cargo_app;
pub mod clone;

pub use application::*;
pub use book::*;
pub use cargo_app::*;
pub use clone::*;

pub trait Search {
    fn search(to_check: &str, object: &Vec<Self>) -> Result<Self, String>
    where
        Self: Sized;
}

pub trait ReadJson {
    fn read_json(path: &str) -> Result<Vec<Self>, std::io::Error>
    where
        Self: Sized;
}

pub trait List {
    fn list(item: &Vec<Self>)
    where
        Self: Sized;
}

pub type Retrieve<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
