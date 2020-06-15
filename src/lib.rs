use std::collections::HashMap;
use std::io::prelude::*;

use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("GitHub responded with a HTTP status of {status}")]
    APIError { status: String },
    #[error("Network and parsing request failed")]
    RequestError(#[from] reqwest::Error),
    #[error("File I/O error occurred")]
    FileIOError(#[from] std::io::Error),
}

#[derive(Deserialize)]
pub struct User {
    pub login: Option<String>,
}

#[derive(Deserialize)]
pub struct File {
    pub raw_url: Option<String>,
    pub filename: Option<String>,
    pub language: Option<String>,
    pub content: Option<String>,
    pub size: usize,
    pub truncated: Option<bool>,
}

#[derive(Deserialize)]
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
    token: String,
) -> Result<Gist, Error> {
    #[derive(Serialize)]
    struct CreateGistFilePayload {
        content: String,
    }

    #[derive(Serialize)]
    struct CreateGistPayload {
        description: String,
        public: bool,
        files: std::collections::HashMap<String, CreateGistFilePayload>,
    }

    let mut payload_map = std::collections::HashMap::new();
    for fp in files {
        let file = std::fs::File::open(&fp).map_err(Error::FileIOError);
        if let Err(e) = file {
            return Err(e);
        }
        let mut gist_data = String::new();
        match file
            .unwrap()
            .read_to_string(&mut gist_data)
            .map_err(Error::FileIOError)
        {
            Err(e) => return Err(e),
            _ => {}
        };
        payload_map.insert(fp, CreateGistFilePayload { content: gist_data });
    }

    let payload = CreateGistPayload {
        description: description.unwrap_or(String::new()),
        public: is_public,
        files: payload_map,
    };

    let url: &str = "https://api.github.com/gists";
    let client = reqwest::Client::new();
    let req = client
        .post(url)
        .json(&payload)
        .header("user-agent", "gstm")
        .header("authorization", format!("token {}", token));

    let res = req.send().await?;

    let s = res.status();
    if s.is_success() {
        res.json::<Gist>().await.map_err(Error::RequestError)
    } else {
        Err(Error::APIError {
            status: format!("{} {}", s.as_str(), s.canonical_reason().unwrap_or("")),
        })
    }
}

pub async fn list(
    by_user: Option<String>,
    _since: Option<DateTime<FixedOffset>>,
) -> Result<Vec<Gist>, Error> {
    let endpoint = match by_user {
        Some(uname) => format!("https://api.github.com/users/{}/gists", uname),
        None => String::from("https://api.github.com/gists/public"),
    };
    let client = Client::new();
    let req = client.get(endpoint.as_str()).header("user-agent", "gstm");
    let res = req.send().await?;

    let s = res.status();
    if s.is_success() {
        res.json::<Vec<Gist>>().await.map_err(Error::RequestError)
    } else {
        Err(Error::APIError {
            status: format!("{} {}", s.as_str(), s.canonical_reason().unwrap_or("")),
        })
    }
}

pub async fn get(_id: String) -> Result<Gist, Error> {
    let endpoint = format!("https://api.github.com/gists/{}", _id);
    let client = Client::new();
    let req = client.get(endpoint.as_str()).header("user-agent", "gstm");
    let res = req.send().await?;
    let s = res.status();
    if s.is_success() {
        res.json::<Gist>().await.map_err(Error::RequestError)
    } else {
        Err(Error::APIError {
            status: format!("{} {}", s.as_str(), s.canonical_reason().unwrap_or("")),
        })
    }
}
