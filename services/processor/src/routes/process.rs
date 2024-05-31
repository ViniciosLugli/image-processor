use crate::repositories::log::LogRepository;
use crate::utils::error::HttpError;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_session::Session;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use database::db::*;
use serde_json::json;
use std::fs;
extern crate photon_rs;

#[derive(MultipartForm)]
pub struct Upload {
	file: TempFile,
}

#[post("/image")]
async fn process_image(
	MultipartForm(form): MultipartForm<Upload>,
	client: web::Data<PrismaClient>,
	session: Session,
	req: HttpRequest,
) -> Result<HttpResponse, HttpError> {
	if !session.entries().contains_key("user_uuid") {
		return Err(HttpError::unauthorized("User is not authenticated."));
	}

	let log_repo = LogRepository::new(client.into_inner());
	const MAX_FILE_SIZE: usize = 1024 * 1024 * 64; // 64 MB

	match form.file.size {
		0 => return Err(HttpError::bad_request("No file was uploaded.")),
		length if length > MAX_FILE_SIZE => {
			dbg!(format!("File size: {} is too large to process for size limit: {}", length, MAX_FILE_SIZE));
			return Err(HttpError::bad_request("File size is too large."));
		}
		_ => {}
	};
	debug!("File size: {}", form.file.size);

	let file_name = format!("{}_{}", uuid::Uuid::new_v4(), form.file.file_name.clone().unwrap_or_default());
	let file_path = format!("/uploads/{}", file_name);

	debug!("File path: {}", file_path);

	fs::copy(&form.file.file, &file_path).expect("Failed to copy file.");
	let mut image: photon_rs::PhotonImage = photon_rs::native::open_image(&file_path).expect("Failed to open image file.");

	photon_rs::monochrome::grayscale(&mut image);

	photon_rs::native::save_image(image.clone(), &file_path).expect("Failed to save image file.");

	let user_uuid = session.get("user_uuid").expect("Failed to get user_uuid").expect("Failed to get user_uuid");
	log_repo.create(LogType::ProcessImage, user_uuid, json!({ "message": "Image uploaded successfully" })).await.unwrap();

	Ok(HttpResponse::Ok().json(&json!({
		"message": "File uploaded successfully.",
		"file_name": form.file.file_name,
		"size": form.file.size,
		"image": image.get_base64()
	})))
}

pub fn init(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/processor").service(process_image));
}
