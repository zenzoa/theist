use std::error::Error;
use bytes::{ Bytes, Buf };
use image::RgbaImage;

use super::{ image_error, parse_pixel_555, parse_pixel_565 };

struct FileHeader {
	pixel_format: u32, // 2 = 555, 3 = 565
	image_count: u16
}

struct ImageHeader {
	first_line_offset: u32,
	width: u16,
	height: u16
}

fn read_file_header(buffer: &mut Bytes) -> Result<FileHeader, Box<dyn Error>> {
	if buffer.remaining() < 6 { return Err(image_error()); }
	let pixel_format = buffer.get_u32_le();
	let _cols = buffer.get_u16_le();
	let _rows = buffer.get_u16_le();
	let image_count = buffer.get_u16_le(); // this should equal cols * rows
	Ok(FileHeader { pixel_format, image_count })
}

fn read_image_header(buffer: &mut Bytes) -> Result<ImageHeader, Box<dyn Error>> {
	if buffer.remaining() < 8 { return Err(image_error()); }
	let first_line_offset = buffer.get_u32_le() + 4;
	let width = buffer.get_u16_le();
	let height = buffer.get_u16_le();
	if width != 128 || height != 128 {
		return Err("Invalid data. All frames in a BLK file must be 128 x 128 px.".into());
	}
	Ok(ImageHeader {
		width,
		height,
		first_line_offset
	})
}

fn read_image_data(contents: &[u8], header: &ImageHeader, pixel_format: u32) -> Result<RgbaImage, Box<dyn Error>> {
	let mut image = RgbaImage::new(header.width as u32, header.height as u32);
	let mut buffer = Bytes::copy_from_slice(contents);
	buffer.advance(header.first_line_offset as usize);
	for y in 0..image.height() {
		for x in 0..image.width() {
			if buffer.remaining() < 2 { return Err(image_error()); }
			let pixel_data = buffer.get_u16_le();
			let color = match pixel_format {
				2 => parse_pixel_555(pixel_data),
				_ => parse_pixel_565(pixel_data)
			};
			image.put_pixel(x, y, color);
		}
	}
	Ok(image)
}

pub fn decode(contents: &[u8]) -> Result<Vec<RgbaImage>, Box<dyn Error>> {
	let mut frames: Vec<RgbaImage> = Vec::new();
	let mut buffer = Bytes::copy_from_slice(contents);
	let file_header = read_file_header(&mut buffer)?;
	let mut image_headers: Vec<ImageHeader> = Vec::new();
	for _ in 0..file_header.image_count {
		if let Ok(image_header) = read_image_header(&mut buffer) {
			image_headers.push(image_header);
		}
	}
	for image_header in image_headers {
		let image = read_image_data(contents, &image_header, file_header.pixel_format)?;
		frames.push(image);
	}
	Ok(frames)
}
