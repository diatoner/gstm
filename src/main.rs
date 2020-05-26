use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct CreateGistFilePayload {
    content: String,
}

#[derive(Serialize, Debug)]
struct CreateGistPayload {
    description: String,
    public: bool,
    files: std::collections::HashMap<String, CreateGistFilePayload>,
}

#[derive(Deserialize, Debug)]
struct CreateGistResponsePayload {
    url: String,
    html_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("test.gist")?;
    let mut gist_data = String::new();
    file.read_to_string(&mut gist_data)?;

    let mut files = std::collections::HashMap::new();
    files.insert(
        String::from("test.gist"),
        CreateGistFilePayload { content: gist_data },
    );

    let payload = CreateGistPayload {
        description: String::from("test gist"),
        public: true,
        files: files,
    };

    let url: &str = "https://api.github.com/gists";
    let client = reqwest::Client::new();
    let token = "56122920292a664576ebd5ded0e381ba88dc7ea0";
    let req = client
        .post(url)
        .json(&payload)
        .header("user-agent", "gst")
        .header("authorization", format!("token {}", token));

    let resp = req.send().await?;

    println!("{:?}", resp.status());
    println!("{:?}", resp.json::<CreateGistResponsePayload>().await?);

    Ok(())
}
