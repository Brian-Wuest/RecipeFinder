use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAuthorizationResponse {
	pub id: i32,
	pub name: String,
	pub classification: i16,
}

impl GetAuthorizationResponse {
	pub fn new(id: i32, name: String, classification: i16) -> Self {
		GetAuthorizationResponse { id, name, classification }
	}
}
