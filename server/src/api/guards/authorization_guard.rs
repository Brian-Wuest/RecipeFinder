use actix_web::guard::{Guard, GuardContext};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

#[derive(Debug, Clone, Default)]
pub struct AuthorizationGuard {
	allow_permission: String,
}

impl AuthorizationGuard {
	pub fn new(allow_permission: String) -> AuthorizationGuard {
		AuthorizationGuard { allow_permission }
	}
}

impl Guard for AuthorizationGuard {
	fn check(&self, request: &GuardContext) -> bool {
		let request_data = request.req_data();
		let details = request_data.get::<AuthDetails<String>>();

		if let Some(actual_details) = details {
			return actual_details.has_any_permission(&[&self.allow_permission]);
		}

		false
	}
}
