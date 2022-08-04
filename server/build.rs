use std::env;
use std::path::Path;

fn main() {
	match env::var("CARGO_MANIFEST_DIR") {
		Ok(path_string) => {
			let output_dir = Path::new(&path_string).join("target").join(env::var("PROFILE").unwrap());
			let input_path = Path::new(&env::current_dir().unwrap()).join("default.json");

			let output_path = Path::new(&output_dir).join("default.json");

			let res = std::fs::copy(input_path, output_path);

			match res {
				Ok(_) => {}
				Err(error) => {
					println!("cargo:warning={:#?}", error)
				}
			}
		}
		Err(_) => {}
	}
}
