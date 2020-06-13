use std::collections::HashMap;
use std::io::prelude::*;

use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub raw_url: Option<String>,
    pub filename: Option<String>,
    pub language: Option<String>,
    pub content: Option<String>,
    pub size: usize,
    pub truncated: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Gist {
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub public: bool,
    pub created_at: Option<String>,
    pub owner: Option<User>,
    pub files: HashMap<String, File>,
}

pub async fn create(
    files: Vec<String>,
    is_public: bool,
    description: Option<String>,
) -> Result<Gist, Box<dyn std::error::Error>> {
    #[derive(Serialize)]
    pub struct CreateGistFilePayload {
        pub content: String,
    }

    #[derive(Serialize)]
    pub struct CreateGistPayload {
        pub description: String,
        pub public: bool,
        pub files: std::collections::HashMap<String, CreateGistFilePayload>,
    }

    let mut payload_map = std::collections::HashMap::new();
    for fp in files {
        let mut file = std::fs::File::open(&fp)?;
        let mut gist_data = String::new();
        file.read_to_string(&mut gist_data)?;
        payload_map.insert(fp, CreateGistFilePayload { content: gist_data });
    }

    let payload = CreateGistPayload {
        description: match description {
            Some(d) => d,
            None => String::from(""),
        },
        public: is_public,
        files: payload_map,
    };

    let url: &str = "https://api.github.com/gists";
    let client = reqwest::Client::new();
    let token = "56122920292a664576ebd5ded0e381ba88dc7ea0";
    let req = client
        .post(url)
        .json(&payload)
        .header("user-agent", "gstm")
        .header("authorization", format!("token {}", token));

    let resp = req.send().await?;

    let json = resp.json::<Gist>().await?;

    Ok(json)
}

pub async fn list(
    by_user: Option<String>,
    _since: Option<DateTime<FixedOffset>>,
) -> reqwest::Result<Vec<Gist>> {
    let endpoint = match by_user {
        Some(uname) => format!("https://api.github.com/users/{}/gists", uname),
        None => String::from("https://api.github.com/gists/public"),
    };
    let client = Client::new();
    let req = client.get(endpoint.as_str()).header("user-agent", "gstm");
    let res = req.send().await?;

    // TODO catch-all handling of API rate limiting, so we can feed through that.
    //  (Can we _attempt_ to parse as our intended result, and if it fails,
    //   then _attempt_ to parse as a rate limiting message?)

    res.json::<Vec<Gist>>().await
}

pub async fn get(_id: String) -> reqwest::Result<Gist> {
    let endpoint = format!("https://api.github.com/gists/{}", _id);
    let client = Client::new();
    let req = client.get(endpoint.as_str()).header("user-agent", "gstm");
    let res = req.send().await?;
    res.json::<Gist>().await
}
