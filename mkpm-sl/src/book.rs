use crate::{List, ReadJson, Retrieve, Search};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::io;
use std::io::{Cursor, Error};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    pub title: String,
    pub short: String,
}

impl List for Book{
    fn list(item: &Vec<Self>) where Self: Sized {
        println!("//// AVAILABLE BOOKS");
        for i in item{
            println!("// {}", i.title);
        }
    }
}

impl ReadJson for Book {
    fn read_json(path: &str) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        let s = std::fs::read_to_string(path)?;
        let v: Vec<Self> = from_str(&s).unwrap();
        return Ok(v);
    }
}

impl Search for Book {
    fn search(to_check: &str, object: &Vec<Self>) -> Result<Self, String>
    where
        Self: Sized,
    {
        for i in object {
            if to_check == i.title {
                return Ok(i.to_owned());
            }
        }
        let msg = format!("Couldn't find book of title: {}", to_check);
        return Err(msg);
    }
}

impl Book {
    pub async fn get(&self) -> Retrieve<()> {
        let link = format!("http://mkproj.com/download/{}", &self.short);
        let resp = reqwest::get(&link).await?;
        let filename = format!("{}.zip", &self.short);
        let mut file = std::fs::File::create(&filename)?;
        let mut content = Cursor::new(resp.bytes().await?);
        io::copy(&mut content, &mut file)?;
        return Ok(());
    }
}
