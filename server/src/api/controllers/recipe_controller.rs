use actix_identity::Identity;
use actix_web::{
	web::{self, Json},
	HttpRequest, Result,
};

use crate::data::recipe::Recipe;

pub struct RecipeController {}

impl RecipeController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(web::resource("/api/recipe").route(web::get().to(RecipeController::index)));
	}

	async fn index(_user: Identity, _req: HttpRequest) -> Result<Json<Vec<Recipe>>> {
		log::info!("Loading all recipes");
		let result = Recipe::load_all_shared_recipes().await;

		Ok(web::Json(result))
	}
}