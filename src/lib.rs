pub mod cli;
pub mod auth;
pub mod responses;

use reqwest::{
    Response,
    header::{
        HeaderMap,
        HeaderValue,
    },
};

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

impl<'a> Harvest {
    const BASE_URL: &'a str = "https://api.harvestapp.com/v2";

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

    async fn get(&self, route: &str) -> reqwest::Result<Response> {
        let uri = format!("{}/{}", Harvest::BASE_URL, route);
        let res = self.client.get(uri)
            .send()
            .await?;

        Ok(res)
    }
    
    pub async fn get_time_entries(&self) -> reqwest::Result<Vec<responses::TimeEntry>> {
        let res: responses::TimeEntries = self.get("/time_entries")
            .await?
            .json()
            .await?;

        Ok(res.time_entries)
    }
}
