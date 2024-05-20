use database::db::*;
use prisma_client_rust::QueryError;
use std::sync::Arc;

pub type Log = log::Data;

pub struct LogRepository {
	client: Arc<PrismaClient>,
}

impl LogRepository {
	pub fn new(client: Arc<PrismaClient>) -> Self {
		Self { client }
	}

	pub async fn create(&self, log_type: LogType, user_uuid: String, data: serde_json::Value) -> Result<Log, QueryError> {
		self.client.log().create(log_type, user::uuid::equals(user_uuid), vec![log::data::set(data)]).exec().await
	}

	pub async fn find_all(&self) -> Result<Vec<Log>, QueryError> {
		self.client.log().find_many(vec![]).exec().await
	}

	pub async fn find_by_user(&self, user_uuid: String) -> Result<Vec<Log>, QueryError> {
		self.client.log().find_many(vec![log::user::is(vec![user::uuid::equals(user_uuid)])]).exec().await
	}
}
