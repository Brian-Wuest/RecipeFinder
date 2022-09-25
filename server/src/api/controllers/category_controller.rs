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
	web::{self, Json, Path},
	HttpRequest, HttpResponse, Responder, Result,
};

use crate::models::authorization_roles::CAT_ADMIN;

pub struct CategoryController {}

impl CategoryController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(
			web::resource("/api/category")
				.route(
					web::get()
						.to(CategoryController::index)
						.guard(AuthorizationGuard::new(CAT_ADMIN.to_string())),
				)
				.route(
					web::put()
						.to(CategoryController::put)
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
					web::post()
						.to(CategoryController::post)
						.guard(AuthorizationGuard::new(CAT_ADMIN.to_string())),
				),
		);
	}

	async fn index(_user: Identity, _req: HttpRequest) -> Result<Json<Vec<GetCategoryResponse>>> {
		let mut result = Vec::new();
		let data_result = SubCategory::load_all_categories().await;

		if !data_result.is_empty() {
			result = GetCategoryResponse::convert_from_data_model(data_result);
		}

		Ok(web::Json(result))
	}

	async fn put(_user: Identity, _req: HttpRequest, form: Json<NewCategoryRequest>) -> Result<Option<Json<GetCategoryResponse>>> {
		println!("Provided Name: {}", form.name);
		println!("Provided Parent: {:?}", form.parent_category_id);

		Ok(None)
	}

	async fn delete(_user: Identity, _req: HttpRequest, id: Path<i32>) -> Result<Option<Json<GetCategoryResponse>>> {
		println!("Provided Id: {}", id);
		Ok(None)
	}

	async fn post(
		_user: Identity,
		_req: HttpRequest,
		id: Path<i32>,
		form: Json<UpdateCategoryRequest>,
	) -> Result<Option<Json<GetCategoryResponse>>> {
		println!("Provided Id: {}", id);
		println!("Provided Name: {}", form.name);
		println!("Provided Parent: {:?}", form.parent_category_id);
		Ok(None)
	}
}
