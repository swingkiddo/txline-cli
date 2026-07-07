use color_eyre::Result;
use reqwest::Client;
use std::sync::Arc;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    config: Arc<Config>,
}

impl ApiClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config: Arc::new(config),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.config.api_host
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    fn apply_auth_headers(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if !self.config.jwt.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.config.jwt));
        }
        if let Some(ref token) = self.config.api_token {
            req = req.header("X-API-Key", token);
        }
        req
    }

    pub fn get(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.api_host, path);
        let req = self.client.get(&url);
        self.apply_auth_headers(req)
    }

    pub fn post(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.api_host, path);
        let req = self.client.post(&url);
        self.apply_auth_headers(req)
    }

    pub async fn send_json<T: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<T> {
        let response = req.send().await?;
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            color_eyre::eyre::bail!("API error {}: {}", status.as_u16(), text);
        }
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub async fn send_text(&self, req: reqwest::RequestBuilder) -> Result<String> {
        let response = req.send().await?;
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            color_eyre::eyre::bail!("API error {}: {}", status.as_u16(), text);
        }
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn post_json<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let req = self.post(path).json(body);
        self.send_json::<R>(req).await
    }

    pub async fn get_json<R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<R> {
        let req = self.get(path);
        self.send_json::<R>(req).await
    }

    pub async fn get_text(&self, path: &str) -> Result<String> {
        let req = self.get(path);
        self.send_text(req).await
    }
}
