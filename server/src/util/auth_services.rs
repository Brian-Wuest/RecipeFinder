use actix_identity::Identity;
use actix_web::{
	web::{self},
	Result,
};
use argon2::{self, Config};
use tiberius::Uuid;

use crate::{models::authorization_roles::SYS_ADMIN, APP_DATA};

pub fn hash_password(password: String) -> Result<String, &'static str> {
	let secret = APP_DATA.web_info.pass_secret.clone();
	let salt = APP_DATA.web_info.pass_salt.clone();

	let config = Config {
		secret: secret.as_bytes(),
		..Default::default()
	};

	argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).map_err(|err| {
		dbg!(err);
		"Internal Server Error"
	})
}

pub fn verify<'a>(hash: &'a str, password: &'a str) -> Result<bool, &'a str> {
	let secret = APP_DATA.web_info.pass_secret.clone();

	argon2::verify_encoded_ext(hash, password.as_bytes(), secret.as_bytes(), &[]).map_err(|err| {
		dbg!(err);
		"Unauthorized"
	})
}

/// Parses an identity to get the workable Uuid.
pub fn parse_user_id_from_identity_option(identity: &Option<Identity>) -> Option<Uuid> {
	if let Some(user_identity) = identity {
		return parse_user_id_from_identity(user_identity);
	}

	None
}

pub fn parse_user_id_from_identity(identity: &Identity) -> Option<Uuid> {
	if let Ok(string_id) = identity.id() {
		if let Ok(user_id) = Uuid::parse_str(&string_id) {
			return Some(user_id);
		}
	}

	None
}

pub fn user_is_admin(permissions: &web::ReqData<Vec<String>>) -> bool {
	permissions.contains(&SYS_ADMIN.to_string())
}
