use serde::{Deserialize, Deserializer, Serialize};

pub fn deserialize_option_ignore_error<'de, T, D>(d: D) -> Result<Option<T>, D::Error>
where
	T: Deserialize<'de>,
	D: Deserializer<'de>,
{
	Ok(T::deserialize(d).ok())
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchRecipeRequest {
	#[serde(rename = "searchText")]
	pub search_text: Option<String>,

	#[serde(rename = "category")]
	#[serde(default, deserialize_with = "deserialize_option_ignore_error")]
	pub category_id: Option<i64>,

	#[serde(rename = "onlyMine")]
	#[serde(default, deserialize_with = "deserialize_option_ignore_error")]
	pub only_mine: Option<bool>,
}
