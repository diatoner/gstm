use std::collections::HashMap;
use std::io::prelude::*;

use chrono::{DateTime, FixedOffset};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(test)]
fn get_api_endpoint() -> String {
    std::env::var("_gstm_mock_api_endpoint").unwrap()
}

#[cfg(test)]
fn set_api_endpoint(uri: String) {
    std::env::set_var("_gstm_mock_api_endpoint", uri.as_str())
}

#[cfg(not(test))]
fn get_api_endpoint() -> String {
    "https://api.github.com".to_string()
}

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

pub fn build_headers(token: Option<String>) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("gstm"));
    if let Some(t) = token {
        let token_string = format!("token {}", t);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(token_string.as_str()).unwrap(),
        );
    };
    log::debug!("Headers build for request: {:?}", headers);
    headers
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
        if let Err(e) = file
            .unwrap()
            .read_to_string(&mut gist_data)
            .map_err(Error::FileIOError)
        {
            return Err(e);
        };
        payload_map.insert(fp, CreateGistFilePayload { content: gist_data });
    }

    let payload = CreateGistPayload {
        description: description.unwrap_or_default(),
        public: is_public,
        files: payload_map,
    };

    let url = format!("{}/gists", get_api_endpoint());
    let client = reqwest::Client::new();
    let req = client
        .post(url.as_str())
        .json(&payload)
        .headers(build_headers(Some(token)));

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
    token: Option<String>,
) -> Result<Vec<Gist>, Error> {
    let endpoint = match by_user {
        Some(uname) => format!("{}/users/{}/gists", get_api_endpoint(), uname),
        None => format!("{}/gists/public", get_api_endpoint()),
    };
    let client = Client::new();
    let req = client.get(endpoint.as_str()).headers(build_headers(token));
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

pub async fn get(_id: String, token: Option<String>) -> Result<Gist, Error> {
    let endpoint = format!("{}/gists/{}", get_api_endpoint(), _id);
    log::debug!("Requesting {:?}", endpoint);
    let client = Client::new();
    let req = client.get(endpoint.as_str()).headers(build_headers(token));
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get() {
        // Test parameters
        let id = "aa5a315d61ae9438b18d".to_string();

        // Expected output
        let mut file = File::open("tests/files/get.json").unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();

        // Mock server
        let mock_server = MockServer::start().await;
        super::set_api_endpoint(mock_server.uri());
        let template = ResponseTemplate::new(200).set_body_raw(content, "application/json");
        println!("{:?}", template);

        Mock::given(method("GET"))
            .and(path(format!("/gists/{}", id)))
            .respond_with(template)
            .mount(&mock_server)
            .await;

        // API call
        super::get(id, None).await.unwrap();
    }
}
