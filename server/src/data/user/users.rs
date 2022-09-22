use serde::Serialize;
use tiberius::{Row, Uuid};

use crate::data::common::{DataElement, DataTools};

#[derive(Debug, Serialize, Clone, Default)]
pub struct User {
	pub id: Uuid,
	pub name: String,
	pub email: String,
	pub password: Option<Vec<u8>>,
}

impl User {
	pub fn new(name: &str, email: &str, password: &[u8]) -> Self {
		let id = Uuid::new_v4();

		User {
			id,
			name: name.to_owned(),
			email: email.to_owned(),
			password: Some(password.to_vec()),
		}
	}

	/// Loads all users registered in the system.
	pub async fn load_all_users() -> Vec<Self> {
		let query = "Select * From dbo.Users;";

		User::load_collection(query).await
	}

	/// Loads a single user by the identifier provided.
	pub async fn load_user_by_id(id: &Uuid) -> Option<Self> {
		let query = "Select * From dbo.Users Where Id = @P1";

		User::load_single_with_params(query, &[&id.to_owned()]).await
	}

	/// Loads a user where the name or email matches the supplied values.
	pub async fn load_user_by_name_or_email(name: &str, email: &str) -> Option<Self> {
		let query = "Select * From dbo.Users Where Name = @P1 OR email = @P2";

		User::load_single_with_params(query, &[&name.to_owned(), &email.to_owned()]).await
	}

	/// Updates the password of an existing user.
	pub async fn update_password(id: &Uuid, new_password: Vec<u8>) -> bool {
		let query = "Update dbo.Users Set Password = @P1 Where ID = @P2";

		match User::insert_with_params(query, &[&new_password, id]).await {
			Ok(_) => true,
			Err(error) => {
				log::error!("Error During User Insert: {}", error);
				false
			}
		}
	}

	pub async fn update_name_email(id: &Uuid, new_name: &str, new_email: &str) -> bool {
		let mut query = "Update dbo.Users SET ".to_owned();
		let mut updated_both = false;
		let mut single_value = new_name;

		if !new_name.is_empty() {
			query += "Name = @P1 ";

			if !new_email.is_empty() {
				updated_both = true;
				query += "Email = @P2 ";
			}
		} else if !new_email.is_empty() {
			query += "Email = @P1 ";
			single_value = new_email;
		}

		if updated_both {
			query += "WHERE ID = @P3";

			match User::insert_with_params(&query, &[&new_name, &new_email, id]).await {
				Ok(_) => return true,
				Err(error) => {
					log::error!("Error During User Update: {}", error);
				}
			};
		} else {
			query += "WHERE ID = @P2";

			match User::insert_with_params(&query, &[&single_value, id]).await {
				Ok(_) => return true,
				Err(error) => {
					log::error!("Error During User Update: {}", error);
				}
			};
		}

		false
	}

	/// Inserts a new user into the database.
	pub async fn insert_new(user: Self) -> bool {
		let query = "EXEC stpCreateUser @P1, @P2, @P3, @P4";

		match User::insert_with_params(query, &[&user.id, &user.name, &user.email, &user.password]).await {
			Ok(_) => true,
			Err(error) => {
				dbg!(&error);
				log::error!("Error During User Insert: {}", error);
				false
			}
		}
	}

	fn load_from_combined_row(identifier: &Uuid, start_index: &mut usize, row: &Row) -> Self {
		User {
			id: *identifier,
			name: DataTools::get_string_and_increment(start_index, row),
			email: DataTools::get_string_and_increment(start_index, row),
			password: DataTools::get_varbinary_as_option_and_increment(start_index, row),
		}
	}
}

impl DataElement for User {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: Uuid = row.get(0).unwrap();

		Some(User::load_from_combined_row(&id, &mut 1, &row))
	}
}
