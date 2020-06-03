use serde::Deserialize;

use chrono::{DateTime, FixedOffset};

use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct Gist {
    pub url: String,
    pub id: String,
    pub description: String,
    pub public: bool,
    pub created_at: String,
    pub owner: User,
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
    res.json::<Vec<Gist>>().await
}
