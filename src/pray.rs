use crate::agent::*;

use std::str;
use bytes::{ Bytes, BytesMut, Buf, BufMut };

struct IntValue {
	name: String,
	value: u32
}

struct StrValue {
	name: String,
	value: String
}

fn write_string(buffer: &mut BytesMut, num_bytes: usize, string: &str) {
	for i in 0..num_bytes {
		if i >= string.len() {
			buffer.put_u8(0);
		} else {
			buffer.put_u8(string.bytes().nth(i).unwrap());
		}
	}
}

fn write_block_header(buffer: &mut BytesMut, block_type: &str, block_name: &String, block_size: u32) {
	write_string(buffer, 4, block_type);
	write_string(buffer, 128, format!("{}\0", block_name).as_str());
	buffer.put_u32_le(block_size); // uncompressed size
	buffer.put_u32_le(block_size); // compressed size
	buffer.put_u32_le(0); // compression flag - it's off, we're not compressing anything
}

fn write_tags_block(buffer: &mut BytesMut, int_values: Vec<IntValue>, str_values: Vec<StrValue>) {
	buffer.put_u32_le(int_values.len() as u32);
	for val in int_values {
		buffer.put_u32_le(val.name.len() as u32);
		write_string(buffer, val.name.len(), val.name.as_str());
		buffer.put_u32_le(val.value);
	}

	buffer.put_u32_le(str_values.len() as u32);
	for val in str_values {
		buffer.put_u32_le(val.name.len() as u32);
		write_string(buffer, val.name.len(), val.name.as_str());
		buffer.put_u32_le(val.value.len() as u32);
		write_string(buffer, val.value.len(), val.value.as_str());
	}
}

fn write_file_block(buffer: &mut BytesMut, filename: &String, data: &Bytes) {
	println!("Write file block for \"{}\"", filename);
	write_block_header(buffer, "FILE", filename, data.len() as u32);
	buffer.extend_from_slice(data);
}

fn write_agent_block(buffer: &mut BytesMut, tag: &AgentTag) {
	println!("Write agent block for \"{}\"", tag.name);

	let block_type = match tag.supported_game {
		SupportedGame::C3 => "AGNT",
		_ => "DSAG"
	};

	let mut int_values: Vec<IntValue> = Vec::new();
	let mut str_values: Vec<StrValue> = Vec::new();

	int_values.push(IntValue{
		name: String::from("Agent Type"),
		value: 0
	});

	if tag.description.len() > 0 {
		str_values.push(StrValue{
			name: String::from("Agent Description"),
			value: tag.description.to_string()
		});
	}

	if let InjectorPreview::Manual { sprite, animation } = &tag.injector_preview {
		println!("  Write injector preview");
		str_values.push(StrValue{
			name: String::from("Agent Animation File"),
			value: format!("{}.c16", sprite)
		});
		str_values.push(StrValue{
			name: String::from("Agent Animation Gallery"),
			value: sprite.to_string()
		});
		str_values.push(StrValue{
			name: String::from("Agent Animation String"),
			value: animation.to_string()
		});
	}

	println!("  Write {} scripts", tag.scripts.len());
	int_values.push(IntValue{
		name: String::from("Script Count"),
		value: tag.scripts.len() as u32
	});
	for (i, script) in tag.scripts.iter().enumerate() {
		str_values.push(StrValue{
			name: format!("Script {}", i + 1),
			value: str::from_utf8(&tag.script_files[0]).unwrap().to_string()
		});
	}

	if let RemoveScript::Manual(remove_script) = &tag.remove_script {
		println!("  Write remove script");
		str_values.push(StrValue{
			name: String::from("Remove script"),
			value: remove_script.to_string()
		});
	}

	let mut dependency_count = 0;

	println!("  Write sprite dependencies");
	for sprite in &tag.sprites {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 2
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: sprite.get_filename()
		});
	}

	println!("  Write background dependencies");
	for background in &tag.backgrounds {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 2
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: background.get_filename()
		});
	}

	println!("  Write sound dependencies");
	for sound in &tag.sounds {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 1
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: sound.get_filename()
		});
	}

	println!("  Write catalogue dependencies");
	for catalogue in &tag.catalogues {
		dependency_count += 1;
		int_values.push(IntValue{
			name: format!("Dependency Category {}", dependency_count),
			value: 7
		});
		str_values.push(StrValue{
			name: format!("Dependency {}", dependency_count),
			value: catalogue.get_filename()
		});
	}

	int_values.push(IntValue{
		name: String::from("Dependency Count"),
		value: dependency_count
	});

	let mut block_buffer = BytesMut::new();
	write_tags_block(&mut block_buffer, int_values, str_values);

	write_block_header(buffer, block_type, &tag.name, block_buffer.len() as u32);
	buffer.unsplit(block_buffer);
}

pub fn encode(tags: &Vec<Tag>) -> Bytes {
	let mut buffer = BytesMut::new();

	let mut files_buffer = BytesMut::new();
	let mut files_written: Vec<String> = Vec::new();

	write_string(&mut buffer, 4, "PRAY");

	for tag in tags {
		match tag {
			Tag::Agent(agent_tag) => {
				// agent info
				write_agent_block(&mut buffer, agent_tag);

				// sprite files
				for (i, data) in agent_tag.sprite_files.iter().enumerate() {
					let filename = agent_tag.sprites.get(i).unwrap().get_filename();
					if !files_written.contains(&filename) {
						write_file_block(&mut files_buffer, &filename, data);
						files_written.push(filename);
					}
				}

				// background files
				for (i, data) in agent_tag.background_files.iter().enumerate() {
					let filename = agent_tag.backgrounds.get(i).unwrap().get_filename();
					if !files_written.contains(&filename) {
						write_file_block(&mut files_buffer, &filename, data);
						files_written.push(filename);
					}
				}

				// sound files
				for (i, data) in agent_tag.sound_files.iter().enumerate() {
					let filename = agent_tag.sounds.get(i).unwrap().get_filename();
					if !files_written.contains(&filename) {
						write_file_block(&mut files_buffer, &filename, data);
						files_written.push(filename);
					}
				}

				// catalogue files
				for (i, data) in agent_tag.catalogue_files.iter().enumerate() {
					let filename = agent_tag.catalogues.get(i).unwrap().get_filename();
					if !files_written.contains(&filename) {
						write_file_block(&mut files_buffer, &filename, data);
						files_written.push(filename);
					}
				}
			},
			_ => ()
		}
	}

	buffer.unsplit(files_buffer);
	return Bytes::copy_from_slice(&buffer);
}
