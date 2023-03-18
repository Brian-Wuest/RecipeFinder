use serde::{Deserialize, Serialize};
use tiberius::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewRecipeRequest {
	pub user_id: Uuid,
	pub name: String,
	pub ingredients: String,
	pub instructions: String,
	pub category_id: i64,
	pub shared: bool,
}
