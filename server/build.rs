use std::env;
use std::path::{Path, PathBuf};

fn main() {
	// Copy the default json from the root. This file contains database and web configuration information.
	if let Ok(path_string) = env::var("CARGO_MANIFEST_DIR") {
		let output_dir = Path::new(&path_string).join("target").join(env::var("PROFILE").unwrap());

		// Copy specific files from the root directory to the build directory.
		let files: Vec<&str> = vec!["default.json", "cert.pem", "key.pem"];

		// Copy the files.
		copy_files(files, &output_dir);
	}
}

fn copy_files(files: Vec<&str>, output_dir: &PathBuf) {
	for file_name in files {
		let input_path = Path::new(&env::current_dir().unwrap()).join(file_name);
		let output_path = Path::new(&output_dir).join(file_name);

		// Always overwrite the file in the build directory.
		let res = std::fs::copy(input_path, output_path);

		match res {
			Ok(_) => {}
			Err(error) => {
				println!("cargo:warning={:#?}", error);
			}
		};
	}
}
