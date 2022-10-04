use crate::{
	api::{
		guards::AuthorizationGuard,
		models::{
			request::category::{NewCategoryRequest, UpdateCategoryRequest},
			response::GetCategoryResponse,
		},
	},
	data::category::SubCategory,
};
use actix_identity::Identity;
use actix_web::{
	error::ErrorNotFound,
	web::{self, Json, Path},
	HttpRequest, Result,
};

use actix_web::error::ErrorBadRequest;

use crate::models::authorization_roles::CAT_ADMIN;

pub struct CategoryController {}

impl CategoryController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(
			web::resource("/api/category")
				// Don't guard on the index as this does not need to be protected.
				.route(web::get().to(CategoryController::index))
				.route(
					web::post()
						.to(CategoryController::post)
						.guard(AuthorizationGuard::new(CAT_ADMIN.to_string())),
				),
		);

		// This is a different route due to the id parameter.
		cfg.service(
			web::resource("/api/category/{id}")
				.route(
					web::delete()
						.to(CategoryController::delete)
						.guard(AuthorizationGuard::new(CAT_ADMIN.to_string())),
				)
				.route(
					web::put()
						.to(CategoryController::put)
						.guard(AuthorizationGuard::new(CAT_ADMIN.to_string())),
				),
		);
	}

	async fn index(_req: HttpRequest) -> Result<Json<Vec<GetCategoryResponse>>> {
		let mut result = Vec::new();
		let data_result = SubCategory::load_all_categories().await;

		if !data_result.is_empty() {
			result = GetCategoryResponse::convert_from_data_model_list(data_result);
		}

		Ok(web::Json(result))
	}

	async fn post(_user: Identity, _req: HttpRequest, form: Json<NewCategoryRequest>) -> Result<Option<Json<GetCategoryResponse>>> {
		let data_result = SubCategory::load_category_by_name_and_parent(&form.name, &form.parent_category_id).await;

		if let Some(_result) = data_result {
			// Found a category which already exists with this name, cannot insert a new one.
			return Err(ErrorBadRequest("Record already exists"));
		}

		let insert_result = SubCategory::insert_category(&form.name, &form.parent_category_id).await;

		if let Some(final_result) = insert_result {
			let response_model = GetCategoryResponse::convert_from_data_model(&final_result);

			return Ok(Some(web::Json(response_model)));
		}

		Ok(None)
	}

	async fn put(
		_user: Identity,
		_req: HttpRequest,
		id: Path<i64>,
		form: Json<UpdateCategoryRequest>,
	) -> Result<Option<Json<GetCategoryResponse>>> {
		let inner_id = id.into_inner();
		let data_result = SubCategory::load_category_by_id(&inner_id).await;

		if let Some(original_result) = data_result {
			// Found the existing category. Try to see if there is another category with this same name but has a different id.
			let other_data_result = SubCategory::load_category_by_name_and_parent(&form.name, &form.parent_category_id).await;

			if let Some(other_result) = other_data_result {
				// There is one which matches, if the IDs match, don't do anything since the id, name and parent match.
				// If the ids don't match, return an error because we cannot make duplicates.
				if other_result.id == original_result.id {
					return Err(ErrorBadRequest("Record already exists"));
				} else {
					// Return null to show that nothing happened.
					return Ok(None);
				}
			} else {
				// A category with this name and parent doesn't exist, okay to update.
				let update_data_result = SubCategory::update_category(&inner_id, &form.name, &form.parent_category_id).await;

				if let Some(update_result) = update_data_result {
					let response_model = GetCategoryResponse::convert_from_data_model(&update_result);

					return Ok(Some(web::Json(response_model)));
				}
			}
		}

		Err(ErrorNotFound("Record not found"))
	}

	async fn delete(_user: Identity, _req: HttpRequest, id: Path<i64>) -> Result<String> {
		let inner_id = id.into_inner();

		let data_result = SubCategory::load_category_by_id(&inner_id).await;

		if let Some(_original_result) = data_result {
			// The category exists, make sure it's not a parent category as sub-categories have to be deleted first.
			let parent_data_result = SubCategory::is_category_a_parent(&inner_id).await;

			if parent_data_result {
				return Err(ErrorBadRequest(
					"Unable to delete category, sub-categories need to be deleted first",
				));
			}

			// The category exists, make sure it doesn't have any associated recipes.
			let recipe_data_result = SubCategory::does_category_have_recipes(&inner_id).await;

			if recipe_data_result {
				return Err(ErrorBadRequest("Unable to delete category, there are associated recipes"));
			}

			// If we got this far, then it is safe to delete this category
			match SubCategory::delete_category_by_id(&inner_id).await {
				Ok(_) => {
					return Ok("{ \"result\": \"Record deleted successfully!\" }".to_string());
				}
				Err(error) => {
					log::warn!("Error deleting category with id: {}, error: {}", &inner_id, error);
				}
			}
		}

		Err(ErrorNotFound("Record not found"))
	}
}
