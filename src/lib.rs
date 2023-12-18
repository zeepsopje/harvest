use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub mod cli;
pub mod auth;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[derive(thiserror::Error, Debug)]
pub enum Error {
}

pub struct Harvest {
    client: reqwest::Client,
}

impl Harvest {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {token}");
        headers.insert(
            "Authorization",
            HeaderValue::from_bytes(auth_value.as_bytes()).unwrap()
        );
        let client = reqwest::ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }
}
