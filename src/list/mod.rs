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
    pub description: Option<String>,
    pub public: bool,
    pub created_at: String,
    pub owner: Option<User>,
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
