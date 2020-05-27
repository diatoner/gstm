use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CreateGistFilePayload {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct CreateGistPayload {
    pub description: String,
    pub public: bool,
    pub files: std::collections::HashMap<String, CreateGistFilePayload>,
}

#[derive(Deserialize, Debug)]
pub struct CreateGistResponsePayload {
    pub url: String,
    pub html_url: String,
}

pub async fn create(
    files: Vec<String>,
    is_public: bool,
    description: String,
) -> Result<CreateGistResponsePayload, Box<dyn std::error::Error>> {
    let mut payload_map = std::collections::HashMap::new();
    for fp in files {
        let mut file = std::fs::File::open(&fp)?;
        let mut gist_data = String::new();
        file.read_to_string(&mut gist_data)?;

        payload_map.insert(fp, CreateGistFilePayload { content: gist_data });
    }

    let payload = CreateGistPayload {
        description,
        public: is_public,
        files: payload_map,
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
    let json = resp.json::<CreateGistResponsePayload>().await?;
    Ok(json)
}
