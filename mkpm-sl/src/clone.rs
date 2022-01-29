use crate::{ReadJson, Search};
use git2::Error as GitError;
use git2::Repository;
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clone {
    pub repo: String,
}

impl ReadJson for Clone {
    fn read_json(path: &str) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
    {
        let s = read_to_string(path)?;
        let v: Vec<Self> = from_str(&s).unwrap();
        return Ok(v);
    }
}

impl Search for Clone {
    fn search(to_check: &str, object: &Vec<Self>) -> Result<Self, String>
    where
        Self: Sized,
    {
        for i in object {
            if i.repo == to_check {
                return Ok(i.to_owned());
            }
        }
        let s = format!("Couldn't find repo with name: {}", to_check);
        return Err(s);
    }
}

impl Clone {
    pub fn clone_repo(&self, path: Option<String>) -> Result<(), GitError> {
        let path = match path {
            Some(s) => s,
            None => "".to_owned(),
        };
        let url = format!("http://github.com/MKProj/{}.git", &self.repo);
        Repository::clone(&url, path)?;
        Ok(())
    }
}
