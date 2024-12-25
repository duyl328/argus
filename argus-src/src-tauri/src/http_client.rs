use crate::conf;
use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        // Client::new();
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(10)) // 请求超时
            .build()
            .expect("Failed to build HTTP client");

        HttpClient { client }
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        Ok(self.client.get(url).send().await?)
    }

    pub async fn post<T>(&self, url: &str, body: &T) -> Result<reqwest::Response>
    where
        T: serde::Serialize,
    {
        Ok(self.client.post(url).json(body).send().await?)
    }
}

pub fn get_base_url() -> String {
    let data = conf::CONF.read().expect("报错了");
    data.python_service_path.clone()
}
