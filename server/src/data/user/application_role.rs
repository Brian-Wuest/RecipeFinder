use serde::{Deserialize, Serialize};
use tiberius::{Row, Uuid};

use crate::data::common::{DataElement, DataTools};
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApplicationRole {
	pub id: i32,
	pub name: String,
	pub classification: i16,
}

impl ApplicationRole {
	pub fn new(id: i32, name: String, classification: i16) -> Self {
		ApplicationRole { id, name, classification }
	}

	/// Loads a single user by the identifier provided.
	pub async fn get_highest_role_for_user(id: &Uuid) -> Option<Self> {
		let query = "exec stpGetMaxRoleForUser @P1";

		ApplicationRole::load_single_with_params(query, &[&id.to_owned()]).await
	}

	pub(crate) fn load_from_combined_row(id: &i32, start_index: &mut usize, row: &Row) -> Self {
		ApplicationRole::new(
			*id,
			DataTools::get_string_and_increment(start_index, row),
			DataTools::get_i16_and_increment(start_index, row),
		)
	}
}

impl DataElement for ApplicationRole {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: i32 = row.get(0).unwrap();

		Some(ApplicationRole::load_from_combined_row(&id, &mut 1, &row))
	}
}
