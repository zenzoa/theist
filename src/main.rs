mod agent;
mod c16;
mod blk;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use image::RgbaImage;
use image::io::Reader as ImageReader;

fn main() {
	let args: Vec<String> = env::args().collect();
	if let (Some(action), Some(filepath)) = (&args.get(1), &args.get(2)) {
		let filepath_pattern = Regex::new(r"^(.*[\\/])?(.+)\.(.+)$").unwrap();
		if let Some(captures) = filepath_pattern.captures(filepath) {

			println!("num captures: {}", captures.len());
			for i in 0..captures.len() {
				if let Some(capture) = captures.get(i) {
					println!("capture: {:?}", capture);
				}
			}

			let path = &captures[1];
			let filename = &captures[2];
			let extension = &captures[3];

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
							let images = c16::decode(&contents);
							for (i, image) in images.iter().enumerate() {
								let output_filepath = format!("{}{}-{}.png", path, filename, i);
								match File::open(&output_filepath) {
									Err(_why) => {
										image.save(&output_filepath);
									},
									Ok(_file) => println!("ERROR: file already exists: {}", &output_filepath)
								}
							}
						},
						Err(error) => {
							println!("ERROR: {}", error)
						}
					}
				},
				"png_to_c16" => {
					let output_filepath = format!("{}{}.c16", path, filename); // TODO: use regex to ignore any end numbers
					match File::open(format!("{}{}.c16", path, filename)) {
						Err(_why) => {
							let mut images: Vec<RgbaImage> = Vec::new();
							for i in 2..args.len() {
								if let Ok(image) = ImageReader::open(&args[i]) {
									if let Ok(image) = image.decode() {
										images.push(image.into_rgba8());
									}
								}
							}
							let c16_data = c16::encode(images);
							match File::create(&output_filepath) {
								Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why),
								Ok(mut file) => {
									file.write_all(&c16_data);
								}
							}
						},
						Ok(_file) => println!("ERROR: file already exists: {}", &output_filepath)
					}
				},
				"blk_to_png" => {
					match fs::read(filepath) {
						Ok(contents) => {
							println!("READ BLK FROM: {}", filepath);
							if let Some(image) = blk::decode(&contents) {
								let output_filepath = format!("{}{}.png", path, filename);
								match File::open(&output_filepath) {
									Err(_why) => {
										image.save(&output_filepath);
									},
									Ok(_file) => println!("ERROR: file already exists: {}", &output_filepath)
								}
							}
						},
						Err(error) => {
							println!("ERROR: {}", error)
						}
					}
				},
				"png_to_blk" => {
					match fs::read(filepath) {
						Ok(contents) => {
							println!("WRITE BLK FROM: {}", filepath);
							if let Ok(image) = ImageReader::open(filepath) {
								if let Ok(image) = image.decode() {
									let blk_data = blk::encode(image.into_rgba8());
									let output_filepath = format!("{}{}.blk", path, filename);
									match File::create(&output_filepath) {
										Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why),
										Ok(mut file) => {
											file.write_all(&blk_data);
										}
									}
								}
							}
						},
						Err(error) => {
							println!("ERROR: {}", error)
						}
					}
				},
				_ => ()
			}
		}
	}
}
