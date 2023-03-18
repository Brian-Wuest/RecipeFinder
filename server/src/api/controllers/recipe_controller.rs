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
		models::{
			request::recipe::{NewRecipeRequest, SearchRecipeRequest, UpdateRecipeRequest},
			response::SearchRecipeResponse,
		},
	},
	data::recipe::Recipe,
	models::authorization_roles::BASIC,
	util::auth_services::{self, parse_user_id_from_identity, parse_user_id_from_identity_option},
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

		cfg.service(web::resource("/api/recipe/_search").route(web::get().to(RecipeController::search)));

		// This is a different route due to the id parameter.
		cfg.service(
			web::resource("/api/recipe/{id}")
				.route(web::get().to(RecipeController::get))
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

		let user_identity = parse_user_id_from_identity_option(&user);

		let result = Recipe::load_all_shared_recipes(user_identity).await;

		Ok(web::Json(result))
	}

	async fn search(
		user: Option<Identity>,
		_req: HttpRequest,
		query_data: Query<SearchRecipeRequest>,
	) -> Result<Option<Json<Vec<SearchRecipeResponse>>>> {
		log::info!("Doing recipe search!");

		let user_identity = parse_user_id_from_identity_option(&user);
		let search_text = &query_data.search_text;
		let category_id = &query_data.category_id;

		let data_result = Recipe::search(&user_identity, search_text, category_id).await;

		if data_result.len() > 0 {
			let result = SearchRecipeResponse::convert_from_data_model_collection(data_result);

			return Ok(Some(web::Json(result)));
		}

		Err(ErrorNotFound("No Recipes found"))
	}

	async fn get(user: Option<Identity>, _req: HttpRequest, id: Path<Uuid>) -> Result<Option<Json<Recipe>>> {
		let user_identity = parse_user_id_from_identity_option(&user);
		let recipe_id = id.into_inner();

		let potential_data_result = Recipe::select_by_id(&user_identity, &recipe_id, &false).await;

		if let Some(data_result) = potential_data_result {
			return Ok(Some(web::Json(data_result)));
		}

		Err(ErrorNotFound("Recipe not found"))
	}

	async fn post(_user: Identity, _req: HttpRequest, _form: Json<NewRecipeRequest>) -> Result<Option<Json<Recipe>>> {
		// Find a recipe by name and this user ID
		// If a recipe matches then return an error as names have to be unique per user.
		// Otherwise add the record for this user and return the created recipe.
		Ok(None)
	}

	async fn put(
		user: Identity,
		permissions: web::ReqData<Vec<String>>,
		_req: HttpRequest,
		id: Path<Uuid>,
		form: Json<UpdateRecipeRequest>,
	) -> Result<Option<Json<Recipe>>> {
		let user_identity = parse_user_id_from_identity(&user);
		let recipe_id = id.into_inner();
		let is_admin_user = auth_services::user_is_admin(&permissions);

		let potential_data_result = Recipe::select_by_id(&user_identity, &recipe_id, &is_admin_user).await;

		if let Some(_data_result) = potential_data_result {
			// The recipe exists and the current user has permissions to it, go ahead and delete it.
			match Recipe::update_by_id(
				&recipe_id,
				&form.name,
				&form.ingredients,
				&form.instructions,
				&form.category_id,
				&form.shared,
			)
			.await
			{
				Some(updated_recipe) => {
					return Ok(Some(web::Json(updated_recipe)));
				}
				None => {
					log::warn!("Error updating recipe with id: {}", &recipe_id);
				}
			}
		}

		Err(ErrorNotFound("Record not found"))
	}

	// The permissions option here is to show how we can get information from the "extensions" part of the request.
	// In this case, the permissions are added as a part of the authorization middle-ware
	// This allows us to avoid another database hit just to re-learn what application permissions the user has.
	async fn delete(user: Identity, permissions: web::ReqData<Vec<String>>, _req: HttpRequest, id: Path<Uuid>) -> Result<String> {
		let user_identity = parse_user_id_from_identity(&user);
		let recipe_id = id.into_inner();
		let is_admin_user = auth_services::user_is_admin(&permissions);

		let potential_data_result = Recipe::select_by_id(&user_identity, &recipe_id, &is_admin_user).await;

		if let Some(_data_result) = potential_data_result {
			// The recipe exists and the current user has permissions to it, go ahead and delete it.
			match Recipe::delete_by_id(&recipe_id).await {
				Ok(_) => {
					return Ok("{ \"result\": \"Record deleted successfully!\" }".to_string());
				}
				Err(error) => {
					log::warn!("Error deleting recipe with id: {}, error: {}", &recipe_id, error);
				}
			}
		}

		Err(ErrorNotFound("Record not found"))
	}
}
