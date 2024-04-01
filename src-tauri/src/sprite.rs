pub mod blk;
pub mod c16;
pub mod s16;

use std::error::Error;
use image::Rgba;
use rfd::FileHandle;

use crate::error_dialog;
use crate::format::file_block::File;

pub fn image_error() -> Box<dyn Error> {
	"Invalid sprite data".into()
}

pub fn parse_pixel_555(pixel: u16) -> Rgba<u8> {
	let r = ((pixel & 0x7c00) >> 7) as u8;
	let g = ((pixel & 0x03e0) >> 2) as u8;
	let b = ((pixel & 0x001f) << 3) as u8;
	Rgba([r, g, b, 255])
}

pub fn parse_pixel_565(pixel: u16) -> Rgba<u8> {
	let r = ((pixel & 0xf800) >> 8) as u8;
	let g = ((pixel & 0x07e0) >> 3) as u8;
	let b = ((pixel & 0x001f) << 3) as u8;
	Rgba([r, g, b, 255])
}

pub fn export_sprite(file: &File, file_handle: &FileHandle, frame_indexes: &[usize]) {
	let decode_result = match file.extension.as_str() {
		"c16" => c16::decode(&file.data),
		"s16" => s16::decode(&file.data),
		"blk" => blk::decode(&file.data),
		_ => Err(image_error())
	};
	match decode_result {
		Ok(frames) => {
			for (i, frame) in frames.iter().enumerate() {
				if frame_indexes.contains(&i) {
					let file_name = match frame_indexes.len() {
						1 => file_handle.file_name(),
						_ => file_handle.file_name().replace(".png", &format!("_{}.png", i))
					};
					let file_path = file_handle.path().with_file_name(file_name);
					if let Err(why) = frame.save(file_path) {
						error_dialog(why.to_string());
					}
				}
			}
		},
		Err(why) => error_dialog(why.to_string())
	}
}
