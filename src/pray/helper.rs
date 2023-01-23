use std::str;
use bytes::{ Bytes, BytesMut, Buf, BufMut };

pub struct BlockHeader {
	pub block_type: String,
	pub name: String,
	pub size: usize,
	pub size_uncompressed: usize,
	pub is_compressed: bool
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

pub fn read_string(buffer: &mut Bytes, num_bytes: usize) -> String {
	let mut string = String::from("");
	for _i in 0..num_bytes {
		let byte = buffer.get_u8();
		if byte != 0 {
			if let Ok(char) = str::from_utf8(&[byte]) {
				string += char;
			}
		}
	}
	string
}

pub fn write_block_header(block_type: &str, block_name: &str, block_size: u32) -> Bytes {
	let mut buffer = BytesMut::new();
	buffer.extend_from_slice(&write_string(block_type, 4));
	buffer.extend_from_slice(&write_string(format!("{}\0", block_name).as_str(), 128));
	buffer.put_u32_le(block_size); // uncompressed size
	buffer.put_u32_le(block_size); // compressed size
	buffer.put_u32_le(0); // compression flag - it's off, we're not compressing anything
	buffer.freeze()
}

pub fn read_block_header(buffer: &mut Bytes) -> BlockHeader {
	BlockHeader {
		block_type: read_string(buffer, 4),
		name: read_string(buffer, 128),
		size: buffer.get_u32_le() as usize,
		size_uncompressed: buffer.get_u32_le() as usize,
		is_compressed: buffer.get_u32_le() == 1
	}
}
