use serde::{Deserialize, Serialize};
use tiberius::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCurrentUserResponse {
	pub id: Option<Uuid>,
}
