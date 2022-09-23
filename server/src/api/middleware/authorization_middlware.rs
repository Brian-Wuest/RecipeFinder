use actix_identity::Identity;
use actix_web::{dev::ServiceRequest, Error};
use tiberius::Uuid;

use crate::{
	data::user::ApplicationRole,
	models::authorization_roles::{BASIC, CAT_ADMIN, POWER_USER, SYS_ADMIN},
};

pub struct AuthorizationMiddlware;

impl AuthorizationMiddlware {
	// You can use custom type instead of String
	pub async fn extract(req: &mut ServiceRequest) -> Result<Vec<String>, Error> {
		let mut return_value: Vec<String> = Vec::new();
		let some_identity = req.extract::<Identity>().await;

		if let Ok(found_identity) = some_identity {
			let user_id = Uuid::parse_str(&found_identity.id().unwrap()).unwrap();
			let data_result = ApplicationRole::get_highest_role_for_user(&user_id).await;

			if let Some(found_result) = data_result {
				if found_result.classification >= 1 {
					return_value.push(BASIC.to_string());
				}

				if found_result.classification >= 2 {
					return_value.push(POWER_USER.to_string());
				}

				if found_result.classification >= 4 {
					return_value.push(CAT_ADMIN.to_string());
				}

				if found_result.classification >= 256 {
					return_value.push(SYS_ADMIN.to_string());
				}
			}
		}

		Result::Ok(return_value)
	}
}
