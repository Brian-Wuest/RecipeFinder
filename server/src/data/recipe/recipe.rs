use crate::data::common::{DataElement, DataTools};
use serde::{Deserialize, Serialize};
use tiberius::{ExecuteResult, Result, Row, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Recipe {
	pub id: Uuid,
	pub user_id: Uuid,
	pub name: String,
	pub ingredients: String,
	pub instructions: String,
	pub category_id: i64,
	pub shared: bool,
}

impl Recipe {
	/// Retrieves all recipes from the system.
	pub async fn load_all_shared_recipes(user_id: Option<Uuid>) -> Vec<Self> {
		let query = "Select * 
      From dbo.Recipe 
      Where Shared = 1
        OR UserID = ISNULL(@p1, '00000000-0000-0000-0000-000000000000');";

		Recipe::load_collection_with_params(query, &[&user_id]).await
	}

	fn load_from_combined_row(id: &Uuid, start_index: &mut usize, row: &Row) -> Self {
		Recipe {
			id: *id,
			user_id: DataTools::get_uuid_and_increment(start_index, row),
			name: DataTools::get_string_and_increment(start_index, row),
			ingredients: DataTools::get_string_and_increment(start_index, row),
			instructions: DataTools::get_string_and_increment(start_index, row),
			category_id: DataTools::get_i64_and_increment(start_index, row),
			shared: DataTools::get_bool_and_increment(start_index, row),
		}
	}

	pub async fn search(user_id: &Option<Uuid>, search_text: &Option<String>, category_id: &Option<i64>) -> Vec<Self> {
		let query = "EXEC stpRecipeSearch @P1, @P2, @P3";

		Recipe::load_collection_with_params(query, &[search_text, category_id, user_id]).await
	}

	pub async fn select_by_id(user_id: &Option<Uuid>, id: &Uuid, is_admin: &bool) -> Option<Recipe> {
		let query = "EXEC stpRecipeSelect01 @P1, @P2, @P3";

		Recipe::load_single_with_params(query, &[id, user_id, is_admin]).await
	}

	pub async fn delete_by_id(id: &Uuid) -> Result<ExecuteResult> {
		let query = "DELETE FROM dbo.Recipe WHERE ID = @P1";

		Recipe::delete_with_params(query, &[id]).await
	}

	pub async fn update_by_id(
		id: &Uuid,
		name: &String,
		ingredients: &String,
		instructions: &String,
		category_id: &i64,
		shared: &bool,
	) -> Option<Recipe> {
		let query = "EXEC stpRecipeUpdate @P1, @P2, @P3, @P4, @P5, @P6";

		Recipe::load_single_with_params(query, &[id, name, ingredients, instructions, category_id, shared]).await
	}
}

impl DataElement for Recipe {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: Uuid = row.get(0).unwrap();

		Some(Recipe::load_from_combined_row(&id, &mut 1, &row))
	}
}
