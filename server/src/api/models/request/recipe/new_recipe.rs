use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewRecipeRequest {
	pub name: String,
	pub ingredients: String,
	pub instructions: String,

	#[serde(rename = "categoryId")]
	pub category_id: i64,
	pub shared: bool,
}
