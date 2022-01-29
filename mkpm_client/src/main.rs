use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use mkpm_sl::clone::Clone;
use structopt::StructOpt;
use mkpm_sl::Application;

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
        repo: String
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
        addr = String::from("mkpm_server.mkproj.com/");
    }
    let args = CLI::from_args();
    match args{
        CLI::Get { name, target} => {
            let client = Client::new();
            let url = format!("{}{}", &addr, &name);
            let request = client.get(&url).send().await.unwrap();
            let app = request.json::<Application>().await.unwrap();
            match app.get(&target).await {
                Ok(b) => b,
                Err(e) => eprintln!("{}", Box::in(e).to_string())
            };
        }
        _ => println!("Other commands")
    }
    Ok(())
}
