use crate::repositories::{log::LogRepository, user::UserRepository};
use crate::utils::error::HttpError;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use database::db::*;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct UserCreateInput {
	name: String,
	email: String,
	password: String,
}

#[derive(Deserialize, Debug)]
struct UserLoginInput {
	email: String,
	password: String,
}

#[post("/register")]
async fn register_user(client: web::Data<PrismaClient>, input: web::Json<UserCreateInput>) -> Result<HttpResponse, HttpError> {
	let user_repo = UserRepository::new(client.clone().into_inner());
	let log_repo = LogRepository::new(client.into_inner());
	let user = match user_repo.create(input.name.clone(), input.email.clone(), input.password.clone()).await {
		Ok(user) => user,
		Err(_) => return Err(HttpError::bad_request("User already exists")),
	};

	log_repo.create(LogType::UserRegister, user.uuid, json!({ "message": "User created successfully" })).await.unwrap();

	Ok(HttpResponse::Created().json(&json!({ "message": "User created successfully"})))
}

#[post("/login")]
async fn login_user(
	client: web::Data<PrismaClient>,
	input: web::Json<UserLoginInput>,
	session: Session,
) -> Result<HttpResponse, HttpError> {
	let user_repo = UserRepository::new(client.clone().into_inner());
	let log_repo = LogRepository::new(client.into_inner());
	let user = user_repo.find_by_credentials(input.email.clone(), input.password.clone()).await.unwrap();

	match user {
		Some(user) => {
			log_repo
				.create(LogType::UserLogin, user.uuid.clone(), json!({ "message": "User logged in successfully" }))
				.await
				.unwrap();

			session.clear();
			session.insert("user_uuid", user.uuid).unwrap();
			Ok(HttpResponse::Ok().json(&json!({ "message": "User logged in successfully"})))
		}
		None => Err(HttpError::unauthorized("Invalid credentials")),
	}
}

#[get("/")]
async fn get_info(client: web::Data<PrismaClient>, session: Session) -> impl Responder {
	let user_repo = UserRepository::new(client.into_inner());

	if !session.entries().contains_key("user_uuid") {
		return HttpResponse::Unauthorized().json(&json!({ "message": "User not logged in" }));
	}

	let user = user_repo
		.find_by_uuid(session.get("user_uuid").expect("Failed to get user_uuid").expect("Failed to get user_uuid"))
		.await
		.unwrap();

	match user {
		Some(user) => HttpResponse::Ok().json(&json!({ "name": user.name, "email": user.email })),
		None => HttpResponse::Unauthorized().json(&json!({ "message": "User not found" })),
	}
}

pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/user").service(register_user).service(login_user).service(get_info));
}
