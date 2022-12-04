mod agent;
mod c16;

use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	if let (Some(action), Some(filepath)) = (&args.get(1), &args.get(2)) {
		match action.as_str() {
			"compile" => {
				match fs::read_to_string(filepath) {
					Ok(contents) => {
						println!("COMPILE AGENT FROM: {}", filepath);
						agent::parse_source(&contents);
					},
					Err(error) => {
						println!("ERROR: {}", error)
					}
				}
			},
			"c16_to_png" => {
				match fs::read(filepath) {
					Ok(contents) => {
						println!("READ C16 FROM: {}", filepath);
						c16::decode(&contents);
					},
					Err(error) => {
						println!("ERROR: {}", error)
					}
				}
			},
			"png_to_c16" => {
				// if any additional args, add to list of filepaths
			},
			_ => ()
		}
	}
}
