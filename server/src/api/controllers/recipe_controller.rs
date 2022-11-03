use actix_identity::Identity;
use actix_web::{
	error::ErrorNotFound,
	web::{self, Json, Path},
	HttpRequest, Result,
};
use tiberius::Uuid;

use crate::{
	api::{
		guards::AuthorizationGuard,
		models::request::recipe::{NewRecipeRequest, UpdateRecipeRequest},
	},
	data::recipe::Recipe,
	models::authorization_roles::BASIC,
	util::auth_services::parse_user_id_from_identity,
};

pub struct RecipeController {}

impl RecipeController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(
			web::resource("/api/recipe")
				.route(
					web::get()
						.to(RecipeController::index)
						.guard(AuthorizationGuard::new(BASIC.to_string())),
				)
				.route(
					web::post()
						.to(RecipeController::post)
						.guard(AuthorizationGuard::new(BASIC.to_string())),
				),
		);

		// This is a different route due to the id parameter.
		cfg.service(
			web::resource("/api/recipe/{id}")
				.route(
					web::delete()
						.to(RecipeController::delete)
						.guard(AuthorizationGuard::new(BASIC.to_string())),
				)
				.route(
					web::put()
						.to(RecipeController::put)
						.guard(AuthorizationGuard::new(BASIC.to_string())),
				),
		);
	}

	async fn index(user: Option<Identity>, _req: HttpRequest) -> Result<Json<Vec<Recipe>>> {
		// Keep this log as an example of how to do logging.
		log::info!("Loading all recipes");

		let user_identity = parse_user_id_from_identity(&user);

		let result = Recipe::load_all_shared_recipes(user_identity).await;

		Ok(web::Json(result))
	}

	async fn post(_user: Identity, _req: HttpRequest, form: Json<NewRecipeRequest>) -> Result<Option<Json<Recipe>>> {
		Ok(None)
	}

	async fn put(_user: Identity, _req: HttpRequest, id: Path<Uuid>, form: Json<UpdateRecipeRequest>) -> Result<Option<Json<Recipe>>> {
		let inner_id = id.into_inner();
		Err(ErrorNotFound("Record not found"))
	}

	async fn delete(_user: Identity, _req: HttpRequest, id: Path<Uuid>) -> Result<String> {
		let inner_id = id.into_inner();

		Err(ErrorNotFound("Record not found"))
	}
}
