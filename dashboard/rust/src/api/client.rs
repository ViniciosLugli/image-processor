use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    env::set_var("RUST_BACKTRACE", "1");
    flutter_rust_bridge::setup_default_user_utils();
}

pub struct Response {
    pub status_code: u16,
    pub body: String,
}
pub struct APIClient {
    base_url: String,
    client: Client,
}

impl APIClient {
    pub fn new(base_url: &str) -> Self {
        println!("Creating new APIClient with base_url: {}", base_url);
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    async fn parse_response(&mut self, response: reqwest::Response) -> Response {
        let status = response.status();
        let raw_json = response.text().await.unwrap().parse().unwrap();
        Response {
            status_code: status.as_u16(),
            body: raw_json,
        }
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        url: String,
        params: Option<serde_json::Value>,
    ) -> Result<reqwest::Response, String> {
        let request_builder = self.client.request(method, &url);

        let request = if let Some(params) = params {
            request_builder.json(&params).build()
        } else {
            request_builder.build()
        }
        .map_err(|e| e.to_string())?;

        self.client
            .execute(request)
            .await
            .map_err(|e| e.to_string())
    }

    async fn send_get_request(&self, url: String) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::GET, url, None).await
    }

    async fn send_post_request(
        &self,
        url: String,
        params: serde_json::Value,
    ) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::POST, url, Some(params))
            .await
    }

    async fn send_put_request(
        &self,
        url: String,
        params: serde_json::Value,
    ) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::PUT, url, Some(params))
            .await
    }

    async fn send_delete_request(&self, url: String) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::DELETE, url, None).await
    }

    pub async fn register_user(&mut self, name: &str, email: &str, password: &str) -> Response {
        let url = format!("{}/user/register", self.base_url);
        let params = json!({
            "name": name,
            "email": email,
            "password": password,
        });
        let response = self.send_post_request(url, params).await;
        self.parse_response(response.unwrap()).await
    }

    pub async fn login_user(&mut self, email: &str, password: &str) -> Response {
        let url = format!("{}/user/login", self.base_url);
        let params = json!({
            "email": email,
            "password": password,
        });
        let response = self.send_post_request(url, params).await;

        self.parse_response(response.unwrap()).await
    }
}
