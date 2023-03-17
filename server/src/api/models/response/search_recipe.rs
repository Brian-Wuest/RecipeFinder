use serde::{Deserialize, Serialize};
use tiberius::Uuid;

use crate::data::recipe::Recipe;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchRecipeResponse {
	pub id: Uuid,

	#[serde(rename = "userId")]
	pub user_id: Uuid,
	pub name: String,

	#[serde(rename = "categoryId")]
	pub category_id: i64,
	pub shared: bool,
}

impl SearchRecipeResponse {
	pub fn new(id: Uuid, name: String, user_id: Uuid, category_id: i64, shared: bool) -> Self {
		SearchRecipeResponse {
			id,
			name,
			user_id,
			category_id,
			shared,
		}
	}

	pub(crate) fn convert_from_data_model_collection(data_model: Vec<Recipe>) -> Vec<Self> {
		let mut result: Vec<Self> = Vec::new();

		// Clone and update the resulting user since the password should not be returned except in special cases.
		for data_model in data_model.iter() {
			let converted_model = SearchRecipeResponse::convert_from_data_model(data_model);

			result.push(converted_model);
		}

		result
	}

	pub(crate) fn convert_from_data_model(data_model: &Recipe) -> Self {
		SearchRecipeResponse::new(
			data_model.id,
			data_model.name.clone(),
			data_model.user_id,
			data_model.category_id,
			data_model.shared,
		)
	}
}
