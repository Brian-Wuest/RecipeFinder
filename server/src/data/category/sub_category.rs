use crate::data::common::{DataElement, DataTools};
use serde::{Deserialize, Serialize};
use tiberius::{ExecuteResult, Result, Row};

use super::Category;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SubCategory {
	pub id: i64,
	pub name: String,
	pub parent_category_id: Option<i64>,
	pub parent_category: Option<Category>,
}

impl SubCategory {
	/// Retrieves all sub-categories from the system.
	pub async fn load_all_categories() -> Vec<Self> {
		let query = "Select * 
      From dbo.Category cat
      LEFT JOIN dbo.Category oc
        ON cat.ParentCategoryID = oc.ID";

		SubCategory::load_collection(query).await
	}

	/// Retrieves a category and it's parent category (if any).
	pub async fn load_category_by_id(id: &i64) -> Option<Self> {
		let query = "Select * 
      From dbo.Category cat
      LEFT JOIN dbo.Category oc
        ON cat.ParentCategoryID = oc.ID
      WHERE cat.ID = @p1";

		SubCategory::load_single_with_params(query, &[&id.to_owned()]).await
	}

	/// Retrieves a category and it's parent category (if any).
	pub async fn load_category_by_name_and_parent(name: &str, parent_category_id: &Option<i64>) -> Option<Self> {
		let query = "EXEC stpSelectCategory @P1, @P2";

		SubCategory::load_single_with_params(query, &[&name.to_owned(), &parent_category_id]).await
	}

	pub async fn does_category_have_recipes(id: &i64) -> bool {
		let query = "Select TOP 1 cat.ID, cat.Name, cat.ParentCategoryID, Parent.ID, Parent.Name
    From dbo.Category cat
    JOIN dbo.Recipe rec
      ON rec.CategoryID = cat.ID
    LEFT JOIN dbo.Category Parent
      ON cat.ParentCategoryID = Parent.ID
    WHERE cat.ID = @P1 OR cat.ParentCategoryID = @P1";

		let result = SubCategory::load_single_with_params(query, &[&id.to_owned()]).await;

		result.is_some()
	}

	pub async fn is_category_a_parent(id: &i64) -> bool {
		let query = "Select TOP 1 cat.ID, cat.Name, cat.ParentCategoryID, Parent.ID, Parent.Name
    From dbo.Category cat
    JOIN dbo.Category Parent
      ON cat.ParentCategoryID = Parent.ID
    WHERE cat.ParentCategoryID = @P1";

		let result = SubCategory::load_single_with_params(query, &[&id.to_owned()]).await;

		result.is_some()
	}

	pub async fn insert_category(name: &str, parent_category_id: &Option<i64>) -> Option<Self> {
		let query = "EXEC stpCreateCategory @P1, @P2";

		SubCategory::load_single_with_params(query, &[&name.to_owned(), &parent_category_id]).await
	}

	pub async fn update_category(id: &i64, name: &str, parent_category_id: &Option<i64>) -> Option<Self> {
		let query = "EXEC stpUpdateCategory @P1, @P2, @P3";

		SubCategory::load_single_with_params(query, &[&id.to_owned(), &name.to_owned(), &parent_category_id]).await
	}

	pub async fn delete_category_by_id(id: &i64) -> Result<ExecuteResult> {
		let query = "DELETE FROM dbo.Category WHERE ID = @P1";

		SubCategory::delete_with_params(query, &[&id.to_owned()]).await
	}

	pub(crate) fn load_from_combined_row(id: &i64, start_index: &mut usize, row: &Row) -> Self {
		let mut parent_category: Option<Category> = None;

		if start_index < &mut (row.len() - 1) {
			let name = DataTools::get_string_and_increment(start_index, row);
			let parent_category_id = DataTools::get_i64_as_option_and_increment(start_index, row);

			// Include the details of the parent category when there are more columns to be retreived.
			if let Some(parent_id) = parent_category_id {
				if start_index < &mut (row.len() - 1) {
					*start_index += 1;
					parent_category = Some(Category::load_from_combined_row(&parent_id, start_index, row));
				}
			}

			SubCategory {
				id: *id,
				name,
				parent_category_id,
				parent_category,
			}
		} else {
			SubCategory::default()
		}
	}
}

impl DataElement for SubCategory {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: i64 = row.get(0).unwrap();

		Some(SubCategory::load_from_combined_row(&id, &mut 1, &row))
	}
}
