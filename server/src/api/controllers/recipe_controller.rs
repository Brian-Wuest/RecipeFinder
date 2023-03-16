use actix_identity::Identity;
use actix_web::{
	error::ErrorNotFound,
	web::{self, Json, Path, Query},
	HttpRequest, Result,
};
use tiberius::Uuid;

use crate::{
	api::{
		guards::AuthorizationGuard,
		models::request::recipe::{NewRecipeRequest, SearchRecipeRequest, UpdateRecipeRequest},
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

		cfg.service(
			web::resource("/api/recipe/_search").route(
				web::get()
					.to(RecipeController::search)
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

	async fn search(_user: Option<Identity>, _req: HttpRequest, query_data: Query<SearchRecipeRequest>) -> Result<Option<Json<Recipe>>> {
		log::info!("Doing recipe search!");
		log::info!("Search Text: {:?}", query_data.search_text);
		log::info!("Search Category: {:?}", query_data.category_id);

		Ok(None)
	}

	async fn post(_user: Identity, _req: HttpRequest, _form: Json<NewRecipeRequest>) -> Result<Option<Json<Recipe>>> {
		Ok(None)
	}

	async fn put(_user: Identity, _req: HttpRequest, id: Path<Uuid>, _form: Json<UpdateRecipeRequest>) -> Result<Option<Json<Recipe>>> {
		let _inner_id = id.into_inner();
		Err(ErrorNotFound("Record not found"))
	}

	async fn delete(_user: Identity, _req: HttpRequest, id: Path<Uuid>) -> Result<String> {
		let _inner_id = id.into_inner();

		Err(ErrorNotFound("Record not found"))
	}
}
