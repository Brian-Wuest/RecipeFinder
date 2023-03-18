use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateRecipeRequest {
	pub name: String,
	pub ingredients: String,
	pub instructions: String,
	pub category_id: i64,
	pub shared: bool,
}
