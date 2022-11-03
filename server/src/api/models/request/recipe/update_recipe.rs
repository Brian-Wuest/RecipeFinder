use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateRecipeRequest {
	name: String,
	ingredients: String,
	instructions: String,
	category_id: i64,
	shared: bool,
}
