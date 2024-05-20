use database::db::*;
use prisma_client_rust::QueryError;
use std::sync::Arc;

extern crate bcrypt;
use bcrypt::{hash, verify};

pub type User = user::Data;
const SALT_ROUNDS: u32 = 4;

pub struct UserRepository {
	client: Arc<PrismaClient>,
}

impl UserRepository {
	pub fn new(client: Arc<PrismaClient>) -> Self {
		Self { client }
	}

	pub async fn find_all(&self) -> Result<Vec<User>, QueryError> {
		self.client.user().find_many(vec![]).exec().await
	}

	pub async fn create(&self, name: String, email: String, password: String) -> Result<User, QueryError> {
		let password = hash(password, SALT_ROUNDS).unwrap();

		self.client.user().create(name, email, password, vec![]).exec().await
	}

	pub async fn find_by_credentials(&self, email: String, password: String) -> Result<Option<User>, QueryError> {
		let user = self.client.user().find_first(vec![user::email::equals(email)]).exec().await?;

		match user {
			Some(user) => {
				if verify(password, &user.password).unwrap() {
					Ok(Some(user))
				} else {
					Ok(None)
				}
			}
			None => Ok(None),
		}
	}

	pub async fn find_by_uuid(&self, uuid: String) -> Result<Option<User>, QueryError> {
		self.client.user().find_unique(user::uuid::equals(uuid)).exec().await
	}
}
