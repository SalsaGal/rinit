use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::env::args;

fn main() {
	let args: Vec<String> = args().collect();
	let template_path_length;

	if let Some(template_path) = args.get(1) {
		let dir = if let Ok(dir) = read_dir(template_path) {
			template_path_length = template_path.len();
			dir
		} else if let Ok(dir) = read_dir(format!("{}/.config/rinit/{}/", home::home_dir().unwrap().to_string_lossy(), template_path)) {
			template_path_length = format!("{}/.config/rinit/{}/", home::home_dir().unwrap().to_string_lossy(), template_path).len();
			dir
		} else {
			panic!("Unable to load `{}`", template_path);
		};

		for file in dir {
			if let Ok(file) = file {
				// Replace variables
				let mut file_content = File::open(file.path()).unwrap();
				let mut contents = String::new();
				file_content.read_to_string(&mut contents).unwrap();

				for arg in 2..args.len() {
					contents = contents.replace(&format!("{{${}}}", arg - 2), &args[arg]);
				}

				// Copy to new directory
				let path = &file.path().to_str().unwrap().to_owned()[template_path_length..];
				let mut destination = File::create(path).unwrap();
				destination.write_all(contents.as_bytes()).unwrap();
			}
		}
	} else {
		eprintln!("Enter a template to use as an argument.");
	}
}
