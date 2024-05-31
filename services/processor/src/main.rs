extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use database::db::*;
use std::env;

mod repositories;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().ok();
	pretty_env_logger::init();
	info!("Starting processer service server...");

	info!("Connecting to database...");
	let client = web::Data::new(PrismaClient::_builder().build().await.expect("Failed to connect to database"));
	info!("Connected to database!");

	info!("Running migrations...");
	#[cfg(debug_assertions)]
	client._db_push().await.expect("Failed to push migrations");
	info!("Migrating database...");

	info!("Connecting to Redis...");
	let redis_store =
		RedisSessionStore::new(env::var("REDIS_URL").expect("REDIS_URL must be set")).await.expect("Failed to connect to Redis");
	info!("Connected to Redis!");

	info!("Processer service running at http://0.0.0.0:3002");

	HttpServer::new(move || {
		App::new()
			.wrap(SessionMiddleware::builder(redis_store.clone(), Key::from(&[0; 64])).cookie_secure(false).build())
			.app_data(client.clone())
			.configure(routes::process::init)
	})
	.bind(("0.0.0.0", 3002))?
	.run()
	.await
}
