use crate::{List, ReadJson, Retrieve, Search};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::io;
use std::io::{Cursor, Error, ErrorKind};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub name: String,
    pub targets: Vec<String>,
}

impl List for Application{
    fn list(item: &Vec<Self>) where Self: Sized {
        println!("//// AVAILABLE APPLICATIONS");
        for i in item{
            println!("// {} => {:?}", &i.name, &i.targets)
        }
    }
}

impl ReadJson for Application {
    fn read_json(path: &str) -> Result<Vec<Self>, Error> {
        let s = std::fs::read_to_string(path)?;
        let v: Vec<Self> = from_str(&s).unwrap();
        Ok(v)
    }
}

impl Search for Application {
    fn search(to_check: &str, object: &Vec<Self>) -> Result<Self, String> {
        for i in object {
            if i.name == to_check {
                return Ok(i.to_owned());
            }
        }
        let msg = format!("Unable to find specified application, {}", to_check);
        return Err(msg);
    }
}

impl Application {
    pub async fn get(&self, target: &str) -> Retrieve<()> {
        let rel_link = format!("http://mkproj.com/releases/{}/{}", &self.name, target);
        for t in &self.targets {
            if target == t {
                let resp = reqwest::get(&rel_link).await?;
                let file_name = format!("{}_{}.zip", &self.name, target);
                let mut file = std::fs::File::create(file_name)?;
                let mut content = Cursor::new(resp.bytes().await?);
                io::copy(&mut content, &mut file)?;
                return Ok(());
            }
        }
        return Err(Box::new(Error::from(ErrorKind::NotFound)));
    }
}
