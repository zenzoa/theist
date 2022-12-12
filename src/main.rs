mod c16;
mod blk;
mod agent;
mod pray;

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
			let path = match captures.get(1) {
				Some(m) => m.as_str(),
				None => ""
			};

			let filename = match captures.get(2) {
				Some(m) => m.as_str(),
				None => ""
			};

			// let _extension = match captures.get(3) {
			// 	Some(m) => m.as_str(),
			// 	None => ""
			// };

			match action.as_str() {
				"compile" => {
					let output_filepath = format!("{}{}.agents", path, filename);
					match File::open(&output_filepath) {
						Err(_why) => {
							match fs::read_to_string(filepath) {
								Ok(contents) => {
									let tags = agent::parse_source(&contents, path);
									println!("");
									let data = agent::compile(tags);
									println!("");
									match File::create(&output_filepath) {
										Ok(mut file) => {
											let result = file.write_all(&data);
											match result {
												Ok(_) => println!("Saved file: {}", &output_filepath),
												Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why)
											}
										},
										Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why)
									}
								},
								Err(why) => println!("ERROR: {}", why)
							}
						},
						Ok(_file) => println!("ERROR: file already exists: {}", &output_filepath)
					}
				},
				"decompile" => {
					match fs::read(filepath) {
						Ok(contents) => {
							let path = format!("{}{} files/", path, filename);
							match fs::create_dir(&path) {
								Ok(()) => {
									let (_tags, files) = agent::decompile(&contents);
									for (filename, data) in files {
										let output_filepath = format!("{}{}", path, filename);
										match File::create(&output_filepath) {
											Ok(mut file) => {
												let result = file.write_all(&data);
												match result {
													Ok(_) => println!("Saved file: {}", &output_filepath),
													Err(why) => println!("ERROR: {} could not be saved: {}", &output_filepath, why)
												}
											},
											Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why)
										}
									}
								},
								Err(why) => println!("ERROR: Unable to create folder {}: {}", path, why)
							}
						},
						Err(why) => println!("ERROR: Unable to read file: {}", why)
					}
				},
				"c16_to_png" => {
					match fs::read(filepath) {
						Ok(contents) => {
							let images = c16::decode(&contents);
							for (i, image) in images.iter().enumerate() {
								let output_filepath = format!("{}{}-{}.png", path, filename, i);
								match File::open(&output_filepath) {
									Ok(_file) => println!("ERROR: File already exists: {}", &output_filepath),
									Err(_why) => {
										let result = image.save(&output_filepath);
										match result {
											Ok(_) => println!("Saved file: {}", &output_filepath),
											Err(why) => println!("ERROR: {} could not be saved: {}", &output_filepath, why)
										}
									},
								}
							}
						},
						Err(why) => println!("ERROR: Unable to read file: {}", why)
					}
				},
				"png_to_c16" => {
					let output_filepath = format!("{}{}.c16", path, filename); // TODO: use regex to ignore any end numbers
					match File::open(&output_filepath) {
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
								Ok(mut file) => {
									let result = file.write_all(&c16_data);
									match result {
										Ok(_) => println!("Saved file: {}", &output_filepath),
										Err(why) => println!("ERROR: {} could not be saved: {}", &output_filepath, why)
									}
								},
								Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why)
							}
							println!("Saved file: {}", &output_filepath);
						},
						Ok(_file) => println!("ERROR: file already exists: {}", &output_filepath)
					}
				},
				"blk_to_png" => {
					let output_filepath = format!("{}{}.png", path, filename);
					match File::open(&output_filepath) {
						Err(_why) => {
							match fs::read(filepath) {
								Ok(contents) => {
									match blk::decode(&contents) {
										Some(image) => {
											let result = image.save(&output_filepath);
											match result {
												Ok(_) => println!("Saved file: {}", &output_filepath),
												Err(why) => println!("ERROR: {} could not be saved: {}", &output_filepath, why)
											}
										},
										None => println!("ERROR: Unable to save file: {}", &output_filepath)
									}
								},
								Err(why) => println!("ERROR: Unable to read file: {}", why)
							}
						},
						Ok(_file) => println!("ERROR: File already exists: {}", &output_filepath)
					}
				},
				"png_to_blk" => {
					let output_filepath = format!("{}{}.blk", path, filename);
					match File::open(&output_filepath) {
						Err(_why) => {
							match ImageReader::open(filepath) {
								Ok(image) => {
									match image.decode() {
										Ok(image) => {
											let blk_data = blk::encode(image.into_rgba8());
											match File::create(&output_filepath) {
												Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why),
												Ok(mut file) => {
													let result = file.write_all(&blk_data);
													match result {
														Ok(_) => println!("Saved file: {}", &output_filepath),
														Err(why) => println!("ERROR: {} could not be saved: {}", &output_filepath, why)
													}
												}
											}
										},
										Err(why) => println!("ERROR: Unable to read file: {}", why)
									}
								},
								Err(why) => println!("ERROR: Unable to open file: {}", why)
							}
						},
						Ok(_file) => println!("ERROR: File already exists: {}", &output_filepath)
					}
				},
				_ => ()
			}
		}
	}
}
