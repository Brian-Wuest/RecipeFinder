use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateCategoryRequest {
	pub name: String,

	#[serde(rename = "parentCategoryId")]
	pub parent_category_id: Option<i64>,
}
