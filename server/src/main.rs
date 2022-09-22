#[macro_use]
extern crate lazy_static;
use crate::api::{AuthorizationController, RecipeController};
use crate::models::config::AppConfig;
use crate::{api::CategoryController, api::UsersController};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{Key, SameSite};
use actix_web::http::header;
use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use futures::executor;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;

// Create modules
mod api;
mod data;
mod models;
mod util;

// Create globals.
lazy_static! {
	// Basic application data.
	#[derive(Debug)]
	pub static ref APP_DATA: AppConfig = load_config();

	// Database connection pool.
	#[derive(Debug)]
	pub static ref DATA_POOL: Pool<ConnectionManager> = load_connection_manager();
}

/// Gets the key from the application configuration or generates a random key if the configuration was not filled in.
/// When generating a random key; it is not persisted and users will not be able to call endpoints after an application restart without re-logging in.
fn get_key() -> Key {
	if APP_DATA.web_info.key.is_empty() {
		// Key not found in the configuration file. Generate a random key.
		let key = generate_key();

		Key::from(&key)
	} else {
		let mut key_vec: Vec<u8> = Vec::new();

		for key_value in APP_DATA.web_info.key.split(',') {
			key_vec.push(key_value.parse().unwrap());
		}

		Key::from(&key_vec)
	}
}

/// This is used to generate a random key for web cookie authorization.
fn generate_key() -> Vec<u8> {
	let key_value = Key::generate();
	let master_value = key_value.master();
	let mut key_string = "".to_string();

	// create a comma delimited string of this master key.
	for (index, value) in master_value.iter().enumerate() {
		key_string = key_string + &value.to_string();

		if index != master_value.len() - 1 {
			key_string += ","
		}
	}

	// Print this out to the console so an admin can copy/paste it into the configuration file.
	// TODO: Persist this to the configuration file automagically.
	println!("Set Environment Var: {}", &key_string);

	// Note: This is only valid for the current process. need to persist this information locally.
	std::env::set_var("sample_auth_key", key_string);

	master_value.to_vec()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// TODO: Set this to WARN or ERROR when in production modes.
	// Note: Setting environment variables like this are only good for the current execution of the application
	// When this application stops and restarts these environment settings but be re-configured if desired.
	std::env::set_var("RUST_LOG", "info");
	env_logger::init();

	let local_host = &APP_DATA.web_info.host;
	let local_port = &APP_DATA.web_info.port.to_string();
	let host_address = local_host.to_owned() + ":" + local_port;

	// Get the key from the application configuration.
	let cookie_key = get_key();

	log::info!("Starting http server: {}", &host_address);

	let config = load_rustls_config();

	// Create the Web Host.
	HttpServer::new(move || {
		App::new()
			.wrap(Compress::default())
			.wrap(Logger::default())
			.wrap(
				IdentityMiddleware::builder()
					// Since we are using cookie sessions there isn't a way to manually expire this without custom middleware.
					//.visit_deadline(Some(std::time::Duration::from_secs(86400)))
					.build(),
			)
			.wrap(
				SessionMiddleware::builder(CookieSessionStore::default(), cookie_key.clone())
					.cookie_same_site(SameSite::None)
					.build(),
			)
			.wrap(
				Cors::default()
					// TODO: Add "allowed clients" to web configuration
					// TODO: Add allowed methods to web configuration
					// TODO: Add allowed request ehaders to web configuration
					.allowed_origin("http://localhost:4200")
					.supports_credentials()
					.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
					.allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
					.allowed_header(header::CONTENT_TYPE)
					.max_age(3600),
			)
			.configure(UsersController::config)
			.configure(CategoryController::config)
			.configure(RecipeController::config)
			.configure(AuthorizationController::config)
	})
	.bind_rustls(host_address, config)?
	.run()
	.await
}

fn load_config() -> AppConfig {
	AppConfig::new()
}

fn load_connection_manager() -> Pool<ConnectionManager> {
	let data_info = APP_DATA.database_info.clone();
	let connection_stuff = executor::block_on(data_info.create_pool());

	connection_stuff.connection_pool
}

/// Example provided by: https://github.com/actix/examples/tree/master/https-tls/rustls
fn load_rustls_config() -> rustls::ServerConfig {
	// init server config builder with safe defaults
	let config = ServerConfig::builder().with_safe_defaults().with_no_client_auth();

	// load TLS key/cert files
	let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
	let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

	// convert files to key/cert objects
	let cert_chain = certs(cert_file).unwrap().into_iter().map(Certificate).collect();
	let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file).unwrap().into_iter().map(PrivateKey).collect();

	// exit if no keys could be parsed
	if keys.is_empty() {
		eprintln!("Could not locate PKCS 8 private keys.");
		std::process::exit(1);
	}

	config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
