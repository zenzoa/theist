pub mod blk;
pub mod c16;
pub mod s16;

use std::error::Error;
use image::Rgba;

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
