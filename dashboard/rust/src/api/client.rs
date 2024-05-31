use reqwest::header::HeaderValue;
use reqwest::multipart;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::fs;

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
    cookies: Option<HeaderValue>,
}

impl APIClient {
    pub fn new(base_url: &str) -> Self {
        println!("Creating new APIClient with base_url: {}", base_url);
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
            cookies: None,
        }
    }

    async fn parse_response(&mut self, response: reqwest::Response) -> Response {
        let status = response.status();
        if let Some(set_cookie) = response.headers().get("set-cookie") {
            self.cookies = Some(set_cookie.clone());
        }

        let raw_json = response.text().await.unwrap().parse().unwrap();
        Response {
            status_code: status.as_u16(),
            body: raw_json,
        }
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn get_cookies(&self) -> Option<String> {
        self.cookies
            .clone()
            .map(|c| c.to_str().unwrap().to_string())
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        url: String,
        params: Option<serde_json::Value>,
    ) -> Result<reqwest::Response, String> {
        let request_builder = self.client.request(method, &url);

        let mut request = if let Some(params) = params {
            request_builder.json(&params).build()
        } else {
            request_builder.build()
        }
        .map_err(|e| e.to_string())?;

        if let Some(cookies) = &self.cookies {
            request
                .headers_mut()
                .insert(reqwest::header::COOKIE, cookies.clone().try_into().unwrap());
        }

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

    pub async fn process_image(&mut self, image_path: &str) -> Response {
        let url = format!("{}/processor/image", self.base_url);
        let file = fs::read(image_path).unwrap();
        let filename = image_path.split("/").last().unwrap();
        let file_part = reqwest::multipart::Part::bytes(file).file_name(filename.to_string());
        let form = multipart::Form::new().part("file", file_part);

        let request_builder = self.client.post(&url).multipart(form);
        let mut request = request_builder.build().unwrap();

        if let Some(cookies) = &self.cookies {
            request.headers_mut().insert(
                reqwest::header::COOKIE,
                HeaderValue::from_str(&cookies.to_str().unwrap().replace("HttpOnly;", "")).unwrap(),
            );
        }

        let response = self.client.execute(request).await.unwrap();

        self.parse_response(response).await
    }
}
