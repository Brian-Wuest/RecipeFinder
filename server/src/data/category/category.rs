use crate::data::common::{DataElement, DataTools};
use serde::{Deserialize, Serialize};
use tiberius::Row;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Category {
	pub id: i64,
	pub name: String,
}

impl Category {
	pub(crate) fn load_from_combined_row(id: &i64, start_index: &mut usize, row: &Row) -> Self {
		Category {
			id: *id,
			name: DataTools::get_string_and_increment(start_index, row),
		}
	}
}

impl DataElement for Category {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: i64 = row.get(0).unwrap();

		Some(Category::load_from_combined_row(&id, &mut 1, &row))
	}
}
