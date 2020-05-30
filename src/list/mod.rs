use chrono::{DateTime, FixedOffset};

use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Gist {
    url: String,
    description: String,
    public: bool,
    // created_at: DateTime<FixedOffset>,
}

pub struct ListResponse {
    endpoint: String,
    since: Option<DateTime<FixedOffset>>,
    items: usize,
}

impl ListResponse {
    fn new(endpoint: String, since: Option<DateTime<FixedOffset>>) -> ListResponse {
        ListResponse {
            endpoint,
            since,
            items: 0,
        }
    }
}

impl Stream for ListResponse {
    type Item = Gist;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        /***
         * TODO Somehow retrieve GitHub Gists at self.endpoint
         * TODO Somehow delineate those into a Vec<Gist>
         * TODO Send those out one by one
         * TODO Send Poll::Ready(None) when we run out
         */
        sleep(Duration::from_secs(1));
        self.items += 1;

        if self.items < 6 {
            Poll::Ready(Some(Gist {
                url: String::from("abc"),
                description: String::from("def"),
                public: true,
            }))
        } else {
            Poll::Ready(None)
        }
    }
}

pub fn list(by_user: Option<String>, since: Option<DateTime<FixedOffset>>) -> ListResponse {
    let endpoint = match by_user {
        Some(uname) => format!("https://api.github.com/users/{}/gists", uname),
        None => String::from("https://api.github.com/gists/public"),
    };
    ListResponse::new(endpoint, since)
}
