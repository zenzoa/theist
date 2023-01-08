mod c16;
mod blk;
mod agent;
mod pray;
mod ui;

use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use regex::Regex;
use image::RgbaImage;
use image::io::Reader as ImageReader;
use iced::{ Application, Settings, window };

pub fn main() -> iced::Result {
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
					output_filepath = Some(Filepath::new(arg));
				} else if !action.is_empty() {
					filepaths.push(Filepath::new(arg));
				} else {
					action = arg.to_string();
				}
			}
		}
	}

	match action.as_str() {
		"" => {
			return start_gui();
		}
		"version" => print_version(),
		"--version" => print_version(),
		"help" => print_help(),
		"--help" => print_help(),
		_ => ()
	}

	if let Some(filepath) = filepaths.get(0) {
		let mut output_path_specified = true;
		let output_filepath = match output_filepath {
			Some(output) => {
				if output.path.is_empty() {
					Filepath {
						path: filepath.path.clone(),
						title: output.title.clone(),
						extension: output.extension
					}
				} else {
					Filepath {
						path: output.path.clone(),
						title: output.title.clone(),
						extension: output.extension
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
				match compile(filepath, &output_filepath.path, &output_filepath.title) {
					Ok(output_filename) => { println!("Saved file {}", output_filename) },
					Err(why) => { println!("ERROR: Cannot save file {}: {}", &output_filepath, why) }
				}
			},

			"decompile" => {
				match decompile(filepath, &output_filepath.path, &output_filepath.title, output_path_specified) {
					Ok(output_folder) => { println!("Saved files to {}", output_folder) },
					Err(why) => { println!("ERROR: Cannot save files: {}", why) }
				}
			},

			"c16_to_png" => {
				match c16_to_png(filepath, &output_filepath.path, &output_filepath.title) {
					Ok(output_filenames) => { println!("Saved files {}", output_filenames) },
					Err(why) => { println!("ERROR: Cannot save file: {}", why) }
				}
			},

			"png_to_c16" => {
				match png_to_c16(&filepaths, &output_filepath.path, &output_filepath.title) {
					Ok(output_filename) => { println!("Saved file {}", output_filename) },
					Err(why) => { println!("ERROR: Cannot save file: {}", why) }
				}
			},

			"blk_to_png" => {
				match blk_to_png(filepath, &output_filepath.path, &output_filepath.title) {
					Ok(output_filename) => { println!("Saved file {}", output_filename) },
					Err(why) => { println!("ERROR: Cannot save file: {}", why) }
				}
			},

			"png_to_blk" => {
				match png_to_blk(filepath, &output_filepath.path, &output_filepath.title) {
					Ok(output_filename) => { println!("Saved file {}", output_filename) },
					Err(why) => { println!("ERROR: Cannot save file: {}", why) }
				}
			},

			_ => ()
		}
	}
	Ok(())
}

fn start_gui() -> iced::Result {
	let settings = Settings::<()> {
		window: window::Settings {
			icon: Some(window::icon::Icon::from_file_data(include_bytes!("../images/theist.png"), None).unwrap()),
			..window::Settings::default()
		},
		exit_on_close_request: false,
		..Default::default()
	};
	ui::Main::run(settings)
}

struct Filepath {
	path: String,
	title: String,
	extension: String
}

impl Filepath {
	fn new(string: &str) -> Filepath {
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
					None => string
				};
				let extension = match captures.get(3) {
					Some(m) => &(m.as_str()[1..]),
					None => ""
				};
				Filepath { path: path.to_string(), title: title.to_string(), extension: extension.to_string() }
			},
			None => {
				Filepath { path: String::from(""), title: string.to_owned(), extension: String::from("") }
			}
		}
	}
}

impl fmt::Display for Filepath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}.{}", &self.path, &self.title, &self.extension)
	}
}

fn print_version() {
	println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn print_help() {
	println!("A tool for making Creatures 3/Docking Station agents.");
	println!();
	println!("Usage: theist [COMMAND] [OPTIONS]");
	println!();
	println!("Commands:");
	println!("  version                     Print version info and exit");
	println!("  help                        List installed commands");
	println!("  compile <FILENAME>          Compile a Theist source file and accompanying files to a .AGENTS file");
	println!("  decompile <FILENAME>        Decompile the files from an .AGENTS file into a folder");
	println!("  c16_to_png <FILENAME>       Convert a .C16 file to one or more .PNG files");
	println!("  png_to_c16 <FILENAME LIST>  Convert one or more .PNG files to a single .C16 file");
	println!("  blk_to_png <FILENAME>       Convert a .BLK file to a .PNG file");
	println!("  png_to_blk <FILENAME>       Convert a .PNG file to a .BLK file");
	println!();
	println!("Options:");
	println!("  --output <FILENAME>         Specify what to name the output file");
}

fn compile(filepath: &Filepath, output_path: &String, output_title: &String) -> Result<String, Box<dyn Error>> {
	let output_filepath = Filepath::new(format!("{}{}.agents", output_path, output_title).as_str());
	match File::open(output_filepath.to_string()) {
		Err(_why) => {
			let contents = fs::read_to_string(filepath.to_string())?;
			let tags = agent::decode::decode_source(&contents, &filepath.path);
			println!();
			let data = agent::compile(tags);
			println!();
			let mut file = File::create(output_filepath.to_string())?;
			file.write_all(&data)?;
			Ok(output_filepath.to_string())
		},
		Ok(_file) => Err(Box::new(io::Error::from(io::ErrorKind::AlreadyExists)))
	}
}

fn decompile(filepath: &Filepath, output_path: &String, output_title: &String, output_path_specified: bool) -> Result<String, Box<dyn Error>> {
	let contents = fs::read(filepath.to_string())?;
	let path = if output_path_specified {
			format!("{}{}/", output_path, output_title)
		} else {
			format!("{}{} files/", filepath.path, filepath.title)
		};
	fs::create_dir(&path)?;
	let files = agent::decompile(&contents, &filepath.title)?;
	for agent::FileData{ name, data } in files {
		let output_filepath = format!("{}{:?}", &path, name);
		let mut file = File::create(output_filepath)?;
		file.write_all(&data)?;
	}
	Ok(path)
}

fn c16_to_png(filepath: &Filepath, output_path: &String, output_title: &String) -> Result<String, Box<dyn Error>> {
	let contents = fs::read(filepath.to_string())?;
	let images = c16::decode(&contents)?;
	for (i, image) in images.iter().enumerate() {
		let output_filename = format!("{}{}-{}.png", &output_path, &output_title, i + 1);
		match File::open(&output_filename) {
			Err(_why) => {
				image.save(&output_filename)?;
			},
			Ok(_file) => {
				return Err(Box::new(io::Error::from(io::ErrorKind::AlreadyExists)));
			}
		}
	}
	Ok(format!("{}{}-1.png to {}{}-{}.png", &output_path, &output_title, &output_path, &output_title, images.len()))
}

fn png_to_c16(filepaths: &Vec<Filepath>, output_path: &String, output_title: &String) -> Result<String, Box<dyn Error>> {
	let output_filepath = Filepath::new(format!("{}{}.c16", output_path, output_title).as_str());
	match File::open(output_filepath.to_string()) {
		Err(_why) => {
			let mut images: Vec<RgbaImage> = Vec::new();
			for image_filepath in filepaths {
				let image_data = ImageReader::open(image_filepath.to_string())?;
				let image = image_data.decode()?;
				images.push(image.into_rgba8());
			}
			let c16_data = c16::encode(images);
			let mut file = File::create(output_filepath.to_string())?;
			file.write_all(&c16_data)?;
			Ok(output_filepath.to_string())
		},
		Ok(_file) => Err(Box::new(io::Error::from(io::ErrorKind::AlreadyExists)))
	}
}

fn blk_to_png(filepath: &Filepath, output_path: &String, output_title: &String) -> Result<String, Box<dyn Error>> {
	let output_filepath = Filepath::new(format!("{}{}.png", output_path, output_title).as_str());
	match File::open(output_filepath.to_string()) {
		Err(_why) => {
			let contents = fs::read(filepath.to_string())?;
			let image = blk::decode(&contents)?;
			image.save(&output_filepath.to_string())?;
			Ok(output_filepath.to_string())
		},
		Ok(_file) => Err(Box::new(io::Error::from(io::ErrorKind::AlreadyExists)))
	}
}

fn png_to_blk(filepath: &Filepath, output_path: &String, output_title: &String) -> Result<String, Box<dyn Error>> {
	let output_filepath = Filepath::new(format!("{}{}.blk", output_path, output_title).as_str());
	match File::open(output_filepath.to_string()) {
		Err(_why) => {
			let image_data = ImageReader::open(filepath.to_string())?;
			let image = image_data.decode()?;
			let blk_data = blk::encode(image.into_rgba8());
			let mut file = File::create(output_filepath.to_string())?;
			file.write_all(&blk_data)?;
			Ok(output_filepath.to_string())
		},
		Ok(_file) => Err(Box::new(io::Error::from(io::ErrorKind::AlreadyExists)))
	}
}
