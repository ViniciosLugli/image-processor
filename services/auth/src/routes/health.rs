use actix_web::{get, web, HttpResponse};
use serde_json::json;

#[get("/")]
async fn health() -> HttpResponse {
	HttpResponse::Ok().json(&json!({ "status": "ok" }))
}

pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/health").service(health));
}
