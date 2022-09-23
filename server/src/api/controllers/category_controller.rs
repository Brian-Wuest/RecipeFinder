use crate::{
	api::{guards::AuthorizationGuard, models::response::GetCategoryResponse},
	data::category::SubCategory,
};
use actix_identity::Identity;
use actix_web::{
	web::{self, Json},
	HttpRequest, Result,
};

use crate::models::authorization_roles::CAT_ADMIN;

pub struct CategoryController {}

impl CategoryController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(
			web::resource("/api/category").route(
				web::get()
					.to(CategoryController::index)
					.guard(AuthorizationGuard::new(CAT_ADMIN.to_string())),
			),
		);
	}

	async fn index(_user: Identity, _req: HttpRequest) -> Result<Json<Vec<GetCategoryResponse>>> {
		log::info!("Loading all categories");
		let mut result = Vec::new();
		let data_result = SubCategory::load_all_categories().await;

		if !data_result.is_empty() {
			result = GetCategoryResponse::convert_from_data_model(data_result);
		}

		Ok(web::Json(result))
	}
}
