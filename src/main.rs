mod error;
mod file_helper;
mod image_format;
mod agent;
mod pray;
mod source;

use crate::pray::compile::compile as compile_pray;
use crate::pray::decompile::decompile as decompile_pray;
use crate::source::encode::encode as encode_source;
use crate::source::decode::decode as decode_source;
use crate::agent::tag::split_tags;

use std::env;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub fn main() {
	let args: Vec<String> = env::args().collect();

	if let Some(command) = args.get(1) {
		match command.as_str() {
			"compile" => {
				if let Some(input_filename) = args.get(2) {
					if let Err(error) = compile(input_filename, args.get(3)) {
						println!("ERROR: {}", error);
					}
				}
			},

			"decompile" => {
				if let Some(input_filename) = args.get(2) {
					if let Err(error) = decompile(input_filename, args.get(3)) {
						println!("ERROR: {}", error);
					}
				}
			},

			"version" => print_version(),

			"help" => print_help(),

			_ => () // TODO: attempt to open given filename with theist ui
		}
	}
}

fn compile(input_filename: &String, output_filename: Option<&String>) -> Result<(), Box<dyn Error>> {
	println!("reading file: {}", input_filename);
	let contents = fs::read_to_string(input_filename)?;
	println!("  success!");

	println!("\ndecoding theist source...");
	let result = decode_source(&contents)?;
	let (tags, mut files) = split_tags(result.tags, result.files);
	println!("  success!");

	println!("\nparsed {} tag(s):", &tags.len());
	for tag in &tags {
		println!("  {} \"{}\"", tag.get_type(), tag.get_name());
	}

	println!("\nparsed {} file(s):", &files.len());
	for file in &mut files {
		println!("  {}", file.get_output_filename());
		file.fetch_data(&file_helper::path(input_filename))?;
	}
	println!("  ...found data for all files :)");

	println!("\ncompiling pray file...");
	let data = compile_pray(&tags, &mut files)?;
	println!("  success!");

	let extension = if tags.len() == 1 { "agent" } else { "agents" };
	let output_filename = match output_filename {
		Some(output_filename) => output_filename.to_string(),
		None => format!("{}{}.{}", file_helper::path(input_filename), file_helper::title(input_filename), extension)
	};

	println!("\nsaving pray file: {}", &output_filename);
	let mut file = File::create(output_filename)?;
	file.write_all(&data)?;
	println!("  success!");

	Ok(())
}

fn decompile(input_filename: &String, output_path: Option<&String>) -> Result<(), Box<dyn Error>> {
	println!("reading file: {}", input_filename);
	let contents = fs::read(input_filename)?;
	let result = decompile_pray(&contents, true)?; // get sprite frames as individual files
	println!("  success!");

	let output_path = match output_path {
		Some(output_path) => output_path.to_string(),
		None => format!("{}{} files/", file_helper::path(input_filename), file_helper::title(input_filename))
	};

	println!("\ncreating output directory: {}", &output_path);
	let _create_dir_result = fs::create_dir(&output_path);
	println!("  success!");

	println!("\nsaving files: {}", &result.files.len());
	for file in &result.files {
		println!("  {}", &file.get_output_filename());
		if let Some(data) = file.get_data() {
			let output_filename = format!("{}{}", &output_path, &file.get_output_filename());
			fs::write(output_filename, &data)?;
		}
	}
	println!("  ...saved all files :)");

	let source_filename = format!("{}{}.the", &output_path, file_helper::title(input_filename));
	println!("\nsaving theist file: {}", &source_filename);
	let result2 = decompile_pray(&contents, false)?; // get sprite frames still bundled together with their sprites
	let source_contents = encode_source(result.tags, result2.files)?;
	fs::write(source_filename, source_contents)?;
	println!("  success!");

	Ok(())
}

fn print_version() {
	println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn print_help() {
	println!("A tool for making Creatures 3/Docking Station agents.");
	println!();
	println!("Usage: theist <command>");
	println!();
	println!("Commands:");
	println!("  version                    Print version info");
	println!("  help                       List installed commands");
	println!("  compile <file> <output>    Compile a Theist source file and accompanying");
	println!("                               files to an AGENTS file (output optional)");
	println!("  decompile <file> <output>  Decompile the files from an AGENTS file into");
	println!("                               a folder (output optional)");
}

