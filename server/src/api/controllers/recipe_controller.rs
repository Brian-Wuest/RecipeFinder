use actix_identity::Identity;
use actix_web::{
	web::{self, Json},
	HttpRequest, Result,
};

use crate::{
	api::guards::AuthorizationGuard, data::recipe::Recipe, models::authorization_roles::BASIC,
	util::auth_services::parse_user_id_from_identity,
};

pub struct RecipeController {}

impl RecipeController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(
			web::resource("/api/recipe").route(
				web::get()
					.to(RecipeController::index)
					.guard(AuthorizationGuard::new(BASIC.to_string())),
			),
		);
	}

	async fn index(user: Option<Identity>, _req: HttpRequest) -> Result<Json<Vec<Recipe>>> {
		log::info!("Loading all recipes");

		let user_identity = parse_user_id_from_identity(&user);

		let result = Recipe::load_all_shared_recipes(user_identity).await;

		Ok(web::Json(result))
	}
}
