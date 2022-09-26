use serde::{Deserialize, Serialize};

use crate::data::category::SubCategory;

use super::data_contracts::category::Category as ContractCategory;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCategoryResponse {
	pub id: i64,
	pub name: String,

	#[serde(rename = "parentCategory")]
	pub parent_category: Option<ContractCategory>,
}

impl GetCategoryResponse {
	pub fn new(id: i64, name: String, parent_category: Option<ContractCategory>) -> Self {
		GetCategoryResponse { id, name, parent_category }
	}

	pub(crate) fn convert_from_data_model_list(data_model: Vec<SubCategory>) -> Vec<Self> {
		let mut result: Vec<Self> = Vec::new();

		// Clone and update the resulting user since the password should not be returned except in special cases.
		for data_model in data_model.iter() {
			let converted_model = GetCategoryResponse::convert_from_data_model(data_model);

			result.push(converted_model);
		}

		result
	}

	pub(crate) fn convert_from_data_model(data_model: &SubCategory) -> Self {
		// Clone and update the resulting user since the password should not be returned except in special cases.
		let mut parent_category: Option<ContractCategory> = None;

		if data_model.parent_category.is_some() {
			parent_category = Some(ContractCategory::convert_from_data_model(
				data_model.parent_category.as_ref().unwrap(),
			));
		}

		GetCategoryResponse::new(data_model.id, data_model.name.clone(), parent_category)
	}
}
