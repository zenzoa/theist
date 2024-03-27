use std::error::Error;
use bytes::{ Bytes, Buf };
use image::RgbaImage;

use super::{ image_error, parse_pixel_555, parse_pixel_565 };

struct FileHeader {
	pixel_format: u32, // 2 = 555, 3 = 565
	image_count: u16
}

struct ImageHeader {
	offset: u32,
	width: u16,
	height: u16
}

fn read_file_header(buffer: &mut Bytes) -> Result<FileHeader, Box<dyn Error>> {
	if buffer.remaining() < 6 { return Err(image_error()); }
	Ok(FileHeader {
		pixel_format: buffer.get_u32_le(),
		image_count: buffer.get_u16_le()
	})
}

fn read_image_header(buffer: &mut Bytes) -> Result<ImageHeader, Box<dyn Error>> {
	if buffer.remaining() < 8 { return Err(image_error()); }
	let offset = buffer.get_u32_le();
	let width = buffer.get_u16_le();
	let height = buffer.get_u16_le();
	Ok(ImageHeader {
		offset,
		width,
		height
	})
}

fn read_image_data(contents: &[u8], header: &ImageHeader, pixel_format: u32) -> Result<RgbaImage, Box<dyn Error>> {
	let mut image = RgbaImage::new(header.width.into(), header.height.into());
	let mut buffer = Bytes::copy_from_slice(contents);
	buffer.advance(header.offset as usize);
	for y in 0..header.height {
		for x in 0..header.width {
			if buffer.remaining() < 2 { return Err(image_error()); }
			let pixel_data = buffer.get_u16_le();
			let mut color = match pixel_format {
				2 => parse_pixel_555(pixel_data),
				_ => parse_pixel_565(pixel_data)
			};
			if color[0] == 0 && color[1] == 0 && color[2] == 0 {
				color[3] = 0;
			}
			image.put_pixel(x.into(), y.into(), color);
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
