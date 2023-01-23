use super::helper::{ write_string, read_string };

use std::collections::HashMap;
use bytes::{ Bytes, BytesMut, Buf, BufMut };

pub struct IntValue(pub String, pub u32);
pub struct StrValue(pub String, pub String);

pub enum InfoValue {
	Int(u32),
	Str(String)
}

pub fn write_info_block(int_values: Vec<IntValue>, str_values: Vec<StrValue>) -> Bytes {
	let mut buffer = BytesMut::new();

	buffer.put_u32_le(int_values.len() as u32);
	for int_val in int_values {
		buffer.put_u32_le(int_val.0.len() as u32);
		buffer.extend_from_slice(&write_string(&int_val.0, int_val.0.len()));
		buffer.put_u32_le(int_val.1);
	}

	buffer.put_u32_le(str_values.len() as u32);
	for str_val in str_values {
		buffer.put_u32_le(str_val.0.len() as u32);
		buffer.extend_from_slice(&write_string(&str_val.0, str_val.0.len()));
		buffer.put_u32_le(str_val.1.len() as u32);
		buffer.extend_from_slice(&write_string(&str_val.1, str_val.1.len()));
	}

	buffer.freeze()
}

pub fn read_info_block(buffer: &mut Bytes) -> HashMap<String, InfoValue> {
	let mut info: HashMap<String, InfoValue> = HashMap::new();

	let int_value_count = buffer.get_u32_le();
	for _i in 0..int_value_count {
		let name_length = buffer.get_u32_le();
		let name = read_string(buffer, name_length as usize);
		let value = buffer.get_u32_le();
		info.insert(name, InfoValue::Int(value));
	}

	let str_value_count = buffer.get_u32_le();
	for _i in 0..str_value_count {
		let name_length = buffer.get_u32_le();
		let name = read_string(buffer, name_length as usize);
		let value_length = buffer.get_u32_le();
		let value = read_string(buffer, value_length as usize);
		info.insert(name, InfoValue::Str(value));
	}

	info
}
