use std::{
	str,
	io::{ Read, Write },
	error::Error,
	collections::HashMap,
	path::{ Path, PathBuf },
	ffi::OsStr
};
use bytes::{ Bytes, BytesMut, Buf, BufMut };
use libflate::zlib;

use super::agent_block::{ Agent, GameSupport, read_agent_block, write_agent_block };
use super::egg_block::{ Egg, read_egg_block, write_egg_block };
use super::gb_block::{ GardenBox, read_gb_block, write_gb_block };
use super::file_block::{ File, write_file_block };
use super::generic_block::{ GenericBlock, write_generic_block };

pub struct Tag {
	pub int_values: HashMap<String, u32>,
	pub str_values: HashMap<String, String>
}

#[derive(Clone, serde::Serialize)]
pub enum Block {
	File(File),
	Agent(Agent),
	Egg(Egg),
	GardenBox(GardenBox),
	Generic(GenericBlock)
}

pub struct BlockHeader {
	pub id: String,
	pub name: String,
	pub size_compressed: usize,
	pub size_uncompressed: usize,
	pub is_compressed: bool
}

pub fn decode(bytes: &[u8]) -> Result<Vec<Block>, Box<dyn Error>> {
	let mut blocks: Vec<Block> = Vec::new();
	let mut buffer = Bytes::copy_from_slice(bytes);
	let _file_id = read_string(&mut buffer, 4)?; // should be "PRAY"
	while !buffer.is_empty() {
		for block in read_block(&mut buffer)? {
			blocks.push(block);
		}
	}
	Ok(blocks)
}

pub fn encode(tags: &Vec<Block>, dependencies: &Vec<File>) -> Result<Bytes, Box<dyn Error>> {
	let mut buffer = BytesMut::new();

	buffer.extend_from_slice(&write_string("PRAY", 4));

	for tag in tags {
		match tag {
			Block::File(file_block) => {
				buffer.extend_from_slice(&write_file_block(file_block)?);
			}
			Block::Agent(agent_block) => {
				buffer.extend_from_slice(&write_agent_block(agent_block, dependencies)?);
			}
			Block::Egg(egg_block) => {
				buffer.extend_from_slice(&write_egg_block(egg_block, dependencies)?);
			}
			Block::GardenBox(gb_block) => {
				buffer.extend_from_slice(&write_gb_block(gb_block, dependencies)?);
			}
			Block::Generic(generic_block) => {
				buffer.extend_from_slice(&write_generic_block(generic_block)?);
			}
		}
	}

	for dependency in dependencies {
		if dependency.extension != "cos" {
			buffer.extend_from_slice(&write_file_block(dependency)?);
		}
	}

	Ok(buffer.freeze())
}

pub fn read_block(buffer: &mut Bytes) -> Result<Vec<Block>, Box<dyn Error>> {
	let block_header = read_block_header(buffer)?;
	let mut block_contents = read_block_contents(buffer, &block_header)?;

	match block_header.id.as_str() {
		"FILE" => {
			let file_path = PathBuf::from(block_header.name);
			let name = file_path.file_stem().unwrap_or(OsStr::new(""));
			let extension = file_path.extension().unwrap_or(OsStr::new(""));
			Ok(vec![Block::File(File {
				name: name.to_str().unwrap_or("").to_string(),
				extension: extension.to_str().unwrap_or("").to_string(),
				data: block_contents,
				is_checked: false
			})])
		}

		"AGNT" => {
			read_agent_block(&mut block_contents, &block_header.name, GameSupport::Creatures3)
		}

		"DSAG" => {
			read_agent_block(&mut block_contents, &block_header.name, GameSupport::DockingStation)
		}

		"EGGS" => {
			read_egg_block(&mut block_contents, &block_header.name)
		}

		"DSGB" => {
			read_gb_block(&mut block_contents, &block_header.name)
		}

		_ => {
			Ok(vec![Block::Generic(GenericBlock {
				id: block_header.id,
				name: block_header.name,
				data: block_contents
			})])
		}
	}
}

pub fn get_dependencies(tag: &Tag) -> Vec<String> {
	let mut dependencies: Vec<String> = Vec::new();
	let num_dependencies = *tag.int_values.get("Dependency Count").unwrap_or(&0) as usize;
	for i in 1..(num_dependencies + 1) {
		let dependency = tag.str_values.get(&format!("Dependency {}", i));
		if let Some(dependecy) = dependency {
			dependencies.push(dependecy.clone());
		}
	}
	dependencies
}

pub fn write_dependencies(int_values: &mut Vec<(String, u32)>, str_values: &mut Vec<(String, String)>, tag_dependencies: &[&File]) {
	int_values.push(("Dependency Count".to_string(), tag_dependencies.len() as u32));
	for (i, dependency) in tag_dependencies.iter().enumerate() {
		str_values.push((format!("Dependency {}", i + 1), dependency.filename()));
		int_values.push((format!("Dependency Category {}", i + 1), match dependency.extension.as_str() {
			"wav" | "mng" => 1, // Sounds
			"c16" | "s16" => 2, // Images
			"gen" | "gno" => 3, // Genetics
			"att" => 4, // Body Data
			"blk" => 6, // Backgrounds
			"catalogue" => 7, // Catalogue
			_ => 0 // main DS directory
		}));
	}
}

pub fn read_block_header(buffer: &mut Bytes) -> Result<BlockHeader, Box<dyn Error>> {
	let id = read_string(buffer, 4)?.to_uppercase();
	let name = read_string(buffer, 128)?;
	let size_compressed = read_u32(buffer)? as usize;
	let size_uncompressed = read_u32(buffer)? as usize;
	let is_compressed = read_u32(buffer)? == 1;
	Ok(BlockHeader { id, name, size_compressed, size_uncompressed, is_compressed })
}

pub fn write_block_header(block_header: BlockHeader) -> Bytes {
	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_string(&block_header.id, 4));
	buffer.extend_from_slice(&write_string(format!("{}\0", block_header.name).as_str(), 128));
	buffer.put_u32_le(block_header.size_compressed as u32);
	buffer.put_u32_le(block_header.size_uncompressed as u32);
	buffer.put_u32_le(if block_header.is_compressed { 1 } else { 0 });
	buffer.freeze()
}

pub fn read_block_contents(buffer: &mut Bytes, block_header: &BlockHeader) -> Result<Bytes, Box<dyn Error>> {
	if buffer.len() >= block_header.size_compressed {
		let mut block_contents = buffer.copy_to_bytes(block_header.size_compressed);
		if block_header.is_compressed {
			let mut decoder = zlib::Decoder::new(block_contents.chunk())?;
			let mut decoded_data = Vec::new();
			decoder.read_to_end(&mut decoded_data)?;
			block_contents = Bytes::from(decoded_data);
		}
		Ok(block_contents)
	} else {
		Err("File ends in the middle of block contents".into())
	}
}

pub fn compress_block_contents(block_contents: &Bytes) -> Result<Bytes, Box<dyn Error>> {
	let mut encoder = zlib::Encoder::new(Vec::new())?;
	encoder.write_all(block_contents)?;
	let compressed_data = encoder.finish().into_result()?;
	Ok(Bytes::from(compressed_data))
}

pub fn read_tag_block(buffer: &mut Bytes) -> Result<Tag, Box<dyn Error>> {
	let mut int_values: HashMap<String, u32> = HashMap::new();
	let num_int_values = read_u32(buffer)?;
	for _i in 0..num_int_values {
		let name_len = read_u32(buffer)?;
		let name = read_string(buffer, name_len as usize)?;
		let value = read_u32(buffer)?;
		println!("{} = {}", &name, &value);
		int_values.insert(name, value);
	}

	let mut str_values: HashMap<String, String> = HashMap::new();
	let num_str_values = read_u32(buffer)?;
	for _i in 0..num_str_values {
		let name_len = read_u32(buffer)?;
		let name = read_string(buffer, name_len as usize)?;
		let value_len = read_u32(buffer)?;
		let value = read_string(buffer, value_len as usize)?;
		println!("{} = {}", &name, &value);
		str_values.insert(name, value);
	}

	Ok(Tag{ int_values, str_values })
}

pub fn write_tag_block(int_values: &[(String, u32)], str_values: &[(String, String)]) -> Bytes {
	let mut buffer = BytesMut::new();

	buffer.put_u32_le(int_values.len() as u32);
	for (int_name, int_value) in int_values {
		buffer.put_u32_le(int_name.len() as u32);
		buffer.extend_from_slice(&write_string(int_name, int_name.len()));
		buffer.put_u32_le(*int_value);
	}

	buffer.put_u32_le(str_values.len() as u32);
	for (str_name, str_value) in str_values {
		buffer.put_u32_le(str_name.len() as u32);
		buffer.extend_from_slice(&write_string(str_name, str_name.len()));
		buffer.put_u32_le(str_value.len() as u32);
		buffer.extend_from_slice(&write_string(str_value, str_value.len()));
	}

	buffer.freeze()
}

pub fn read_u32(buffer: &mut Bytes) -> Result<u32, Box<dyn Error>> {
	if buffer.len() >= 4 {
		Ok(buffer.get_u32_le())
	} else {
		Err("File ends in the middle of an integer".into())
	}
}

pub fn read_string(buffer: &mut Bytes, str_len: usize) -> Result<String, Box<dyn Error>> {
	if buffer.len() >= str_len {
		let mut string = String::from("");
		for _i in 0..str_len {
			let byte = buffer.get_u8();
			if byte != 0 {
				if let Ok(char) = str::from_utf8(&[byte]) {
					string += char;
				}
			}
		}
		Ok(string)
	} else {
		Err("File ends in the middle of a string".into())
	}
}

pub fn write_string(string: &str, num_bytes: usize) -> Bytes {
	let mut buffer = BytesMut::new();
	for i in 0..num_bytes {
		if i >= string.len() {
			buffer.put_u8(0);
		} else {
			buffer.put_u8(*string.as_bytes().get(i).unwrap());
		}
	}
	buffer.freeze()
}

pub fn file_stem(file_name: &str) -> String {
	Path::new(file_name).file_stem().unwrap_or_default().to_str().unwrap_or_default().to_string()
}
