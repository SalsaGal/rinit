use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::env::args;

fn main() {
	let args: Vec<String> = args().collect();

	if let Some(template_path) = args.get(1) {
		if let Ok(dir) = read_dir(template_path) {
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
					let mut destination = File::create(&file.path().to_str().unwrap().to_owned()[template_path.len()..]).unwrap();
					destination.write_all(contents.as_bytes()).unwrap();
				}
			}
		} else {
			eprintln!("Unable to load `{}`", template_path);
		}
	} else {
		eprintln!("Enter a template to use as an argument.");
	}
}
