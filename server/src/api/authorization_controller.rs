use actix_identity::Identity;
use actix_web::{
	web::{self, Json},
	HttpRequest, Result,
};
use tiberius::Uuid;

use crate::models::{data::ApplicationRole, response::GetAuthorizationResponse};

pub struct AuthorizationController {}

impl AuthorizationController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(web::resource("/api/authorization").route(web::get().to(AuthorizationController::get_user_authorization)));
	}

	async fn get_user_authorization(user: Identity, _req: HttpRequest) -> Result<Json<GetAuthorizationResponse>> {
		let user_id = Uuid::parse_str(&user.id().unwrap()).unwrap();
		log::info!("Getting authorization for user {}", user_id);
		let mut result = GetAuthorizationResponse::default();
		let data_result = ApplicationRole::get_highest_role_for_user(&user_id).await;

		if let Some(final_result) = data_result {
			result = GetAuthorizationResponse::new(final_result.id, final_result.name, final_result.classification);
		}

		Ok(web::Json(result))
	}
}
