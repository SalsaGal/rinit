use std::fs::{copy, read_dir};
use std::env::args;

fn main() {
	let args: Vec<String> = args().collect();

	if let Some(template_path) = args.get(1) {
		if let Ok(dir) = read_dir(template_path) {
			for file in dir {
				if let Ok(file) = file {
					let destination = file.path().to_str().unwrap().to_owned();
					copy(file.path(), &destination[template_path.len()..]).unwrap();
				}
			}
		} else {
			eprintln!("Unable to load `{}`", template_path);
		}
	} else {
		eprintln!("Enter a template to use as an argument.");
	}
}
