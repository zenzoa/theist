mod c16;
mod blk;
mod agent;
mod pray;

use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use image::RgbaImage;
use image::io::Reader as ImageReader;

struct Filepath {
	path: String,
	title: String,
	extension: String
}

impl Filepath {
	fn new(string: &String) -> Filepath {
		let filepath_pattern = Regex::new(r"(.*[\\/])?([^\.]+)(\..*)?").unwrap();
		let captures = filepath_pattern.captures(string);
		match captures {
			Some(captures) => {
				let path = match captures.get(1) {
					Some(m) => m.as_str(),
					None => ""
				};
				let title = match captures.get(2) {
					Some(m) => m.as_str(),
					None => string.as_str()
				};
				let extension = match captures.get(3) {
					Some(m) => &(m.as_str()[1..]),
					None => ""
				};
				return Filepath { path: path.to_string(), title: title.to_string(), extension: extension.to_string() };
			},
			None => {
				return Filepath { path: String::from(""), title: string.clone(), extension: String::from("") };
			}
		}
	}

	fn to_string(&self) -> String {
		format!("{}{}.{}", &self.path, &self.title, &self.extension)
	}
}

impl fmt::Display for Filepath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}.{}", &self.path, &self.title, &self.extension)
	}
}

fn main() {
	let mut action = String::from("");
	let mut filepaths: Vec<Filepath> = Vec::new();
	let mut output_filepath: Option<Filepath> = None;

	let mut get_output = false;

	let args: Vec<String> = env::args().collect();
	for arg in args[1..].iter() {
		match arg.as_str() {
			"--output" => {
				get_output = true;
			},
			_ => {
				if get_output {
					output_filepath = Some(Filepath::new(&arg));
				} else if action.len() > 0 {
					filepaths.push(Filepath::new(&arg));
				} else {
					action = arg.to_string();
				}
			}
		}
	}

	if let Some(filepath) = filepaths.get(0) {
		let mut output_path_specified = true;
		let mut output_filepath = match output_filepath {
			Some(output) => {
				if output.path.len() == 0 {
					Filepath {
						path: filepath.path.clone(),
						title: output.title.clone(),
						extension: output.extension.clone()
					}
				} else {
					Filepath {
						path: output.path.clone(),
						title: output.title.clone(),
						extension: output.extension.clone()
					}
				}
			},
			None => {
				output_path_specified = false;
				Filepath {
					path: filepath.path.clone(),
					title: filepath.title.clone(),
					extension: filepath.extension.clone()
				}
			}
		};

		match action.as_str() {
			"compile" => {
				output_filepath.extension = String::from("agents");
				match File::open(&output_filepath.to_string()) {
					Err(_why) => {
						match fs::read_to_string(filepath.to_string()) {
							Ok(contents) => {
								let tags = agent::parse_source(&contents, &filepath.path);
								println!("");
								let data = agent::compile(tags);
								println!("");
								match File::create(&output_filepath.to_string()) {
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
				match fs::read(filepath.to_string()) {
					Ok(contents) => {
						let path = if output_path_specified {
							format!("{}{}/", &output_filepath.path, &output_filepath.title)
							} else {
								format!("{}{} files/", &filepath.path, &filepath.title)
							};
						match fs::create_dir(&path) {
							Ok(()) => {
								let files = agent::decompile(&contents, &filepath.title);
								for (filename, data) in files {
									let output_path = format!("{}{}", &path, filename);
									match File::create(&output_path) {
										Ok(mut file) => {
											let result = file.write_all(&data);
											match result {
												Ok(_) => println!("Saved file: {}", &output_path),
												Err(why) => println!("ERROR: {} could not be saved: {}", &output_path, why)
											}
										},
										Err(why) => println!("ERROR: {} cannot be created: {}", &output_path, why)
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
				match fs::read(filepath.to_string()) {
					Ok(contents) => {
						let images = c16::decode(&contents);
						for (i, image) in images.iter().enumerate() {
							let output_filename = format!("{}{}-{}.png", &output_filepath.path, &output_filepath.title, i);
							match File::open(&output_filename) {
								Ok(_file) => println!("ERROR: File already exists: {}", &output_filename),
								Err(_why) => {
									let result = image.save(&output_filename);
									match result {
										Ok(_) => println!("Saved file: {}", &output_filename),
										Err(why) => println!("ERROR: {} could not be saved: {}", &output_filename, why)
									}
								},
							}
						}
					},
					Err(why) => println!("ERROR: Unable to read file: {}", why)
				}
			},

			"png_to_c16" => {
				output_filepath.extension = String::from("c16");
				match File::open(&output_filepath.to_string()) {
					Err(_why) => {
						let mut images: Vec<RgbaImage> = Vec::new();
						for image_filepath in filepaths {
							if let Ok(image) = ImageReader::open(image_filepath.to_string()) {
								if let Ok(image) = image.decode() {
									images.push(image.into_rgba8());
								}
							}
						}
						let c16_data = c16::encode(images);
						match File::create(&output_filepath.to_string()) {
							Ok(mut file) => {
								let result = file.write_all(&c16_data);
								match result {
									Ok(_) => println!("Saved file: {}", &output_filepath),
									Err(why) => println!("ERROR: {} could not be saved: {}", &output_filepath, why)
								}
							},
							Err(why) => println!("ERROR: {} cannot be created: {}", &output_filepath, why)
						}
					},
					Ok(_file) => println!("ERROR: file already exists: {}", &output_filepath)
				}
			},

			"blk_to_png" => {
				output_filepath.extension = String::from("png");
				match File::open(&output_filepath.to_string()) {
					Err(_why) => {
						match fs::read(filepath.to_string()) {
							Ok(contents) => {
								match blk::decode(&contents) {
									Some(image) => {
										let result = image.save(&output_filepath.to_string());
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
				output_filepath.extension = String::from("blk");
				match File::open(&output_filepath.to_string()) {
					Err(_why) => {
						match ImageReader::open(filepath.to_string()) {
							Ok(image) => {
								match image.decode() {
									Ok(image) => {
										let blk_data = blk::encode(image.into_rgba8());
										match File::create(&output_filepath.to_string()) {
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
