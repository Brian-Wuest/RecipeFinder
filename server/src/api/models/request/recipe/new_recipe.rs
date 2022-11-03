use serde::{Deserialize, Serialize};
use tiberius::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewRecipeRequest {
	user_id: Uuid,
	name: String,
	ingredients: String,
	instructions: String,
	category_id: i64,
	shared: bool,
}
