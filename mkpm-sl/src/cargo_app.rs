use crate::{List, ReadJson, Search};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::read_to_string;
use std::io::Error;
use std::process::Command;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CargoApp {
    pub name: String,
}

impl List for CargoApp{
    fn list(item: &Vec<Self>) where Self: Sized {
        println!("//// AVAILABLE CARGO APPS");
        for i in item{
            println!("// {}", i.name);
        }
    }
}


impl Search for CargoApp {
    fn search(to_check: &str, object: &Vec<Self>) -> Result<Self, String>
    where
        Self: Sized,
    {
        for i in object {
            if i.name == to_check {
                return Ok(i.to_owned());
            }
        }
        let msg = format!(
            "Couldn't find MKProject Cargo application with name: {}",
            to_check
        );
        return Err(msg);
    }
}

impl ReadJson for CargoApp {
    fn read_json(path: &str) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        let s = read_to_string(path)?;
        let v: Vec<Self> = from_str(&s).unwrap();
        return Ok(v);
    }
}

impl CargoApp {
    pub fn install(&self) -> Result<(), Error> {
        Command::new("cargo")
            .arg("install")
            .arg(&self.name)
            .spawn()?;
        return Ok(());
    }
}
