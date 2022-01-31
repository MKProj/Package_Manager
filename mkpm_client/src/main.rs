use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use mkpm_sl::clone::Clone;
use structopt::StructOpt;
use mkpm_sl::{Application, Book, CargoApp, List};

#[derive(StructOpt)]
#[structopt(
    name = "MKPM Client",
    about = "Client for MKProject's Package Manager"
)]
enum CLI{
    Get{
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        target: String
    },
    Install{
        #[structopt(short, long)]
        name: String
    },
    Read{
        #[structopt(short, long)]
        title: String
    },
    Clone{
        #[structopt(short, long)]
        repo: String,
        #[structopt(short, long)]
        path: Option<PathBuf>
    },
    List{
        #[structopt(short, long)]
        cmd: String
    }
}
#[derive(Deserialize, Serialize)]
pub struct Address{
    pub address: String
}
/*
let client = reqwest::Client::new();
    let get =  client.get("http://127.0.0.1:8000/clone/MKProj")
        .send()
        .await.unwrap();
    let g= get.json::<Clone>().await.unwrap();
    g.clone_repo(Some("tmp/".to_string()))?;
 */

#[tokio::main]
async fn main() -> Result<(), Error>{
    let path = Path::new("client.toml");
    let mut addr = String::new();
    if path.exists(){
        let s = read_to_string("client.toml")?;
        let a: Address = toml::from_str(&s).unwrap();
        addr = a.address;
    } else {
        addr = String::from("mkpm-server.mkproj.com/");
    }
    let args = CLI::from_args();
    match args{
        CLI::Get { name, target} => {
            let client = Client::new();
            let url = format!("{}get/{}", &addr,&name);
            println!("Sending request to server on {}", &url);
            let request = client.get(&url).send().await.unwrap();
            let app = request.json::<Application>().await.unwrap();
            println!("Getting application...");
            app.get(&target).await.expect("Couldn't get application");
            println!("{}_{}.zip has been downloaded", &name, &target);
        }
        CLI::Install {name} => {
            let client = Client::new();
            let url = format!("{}install/{}", &addr, &name);
            println!("Sending request to server on {}", &url);
            let request = client.get(&url).send().await.unwrap();
            let cargo = request.json::<CargoApp>().await.unwrap();
            cargo.install()?;
        }
        CLI::Read {title} => {
            let client = Client::new();
            let url = format!("{}read/{}", &addr, &title);
            println!("Sending request to server on {}", &url);
            let request = client.get(&url).send().await.unwrap();
            let book = request.json::<Book>().await.unwrap();
            book.get().await.unwrap();
            println!("{}.zip has been downloaded", &book.short);
        }
        CLI::Clone {repo, path} => {
            let path = match path {
              Some(a) => a,
                None => Path::new(".").to_path_buf()
            };

            let client = Client::new();
            let url = format!("{}clone/{}", &addr, &repo);
            println!("Sending request to server on {}", &url);
            let request = client.get(&url).send().await.unwrap();
            let rep = request.json::<Clone>().await.unwrap();
            rep.clone_repo(Some(path.to_str().unwrap().to_string())).unwrap();
            println!("The {} repo has been cloned to {}/{}", &repo, path.to_str().unwrap(), &repo)
        }
        CLI::List {cmd} => {
            let client = Client::new();
            let url = format!("{}{}/list", &addr, &cmd);
            println!("Sending request to server on {}", &url);
            let request = client.get(&url).send().await.unwrap();
            match cmd.as_str() {
                "read" => {
                    let rep = request.json::<Vec<Book>>().await.unwrap();
                    Book::list(&rep);
                }
                "get" => {
                    let rep = request.json::<Vec<Application>>().await.unwrap();
                    Application::list(&rep);
                }
                "install" => {
                    let rep = request.json::<Vec<CargoApp>>().await.unwrap();
                    CargoApp::list(&rep);
                }
                "clone" => {
                    let rep = request.json::<Vec<Clone>>().await.unwrap();
                    Clone::list(&rep);
                }
                _ => return Err(std::io::Error::from(ErrorKind::NotFound))
            };
        }
    }
    Ok(())
}
