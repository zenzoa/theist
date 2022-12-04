use bytes::{ Bytes, Buf };
use image::{ RgbaImage, Rgba };

struct FileHeader {
	pixel_format: u32, // 2 = 555, 3 = 565
	image_count: u16
}

struct ImageHeader {
	width: u16,
	height: u16,
	line_offsets: Vec<u32>
}

fn read_file_header(buffer: &mut Bytes) -> Option<FileHeader> {
	if buffer.remaining() >= 6 {
		return Some(FileHeader {
			pixel_format: buffer.get_u32_le(),
			image_count: buffer.get_u16_le()
		});
	}
	return None;
}

fn read_image_header(buffer: &mut Bytes) -> Option<ImageHeader> {
	if buffer.remaining() >= 8 {
		let mut line_offsets = vec![ buffer.get_u32_le() ];
		let width = buffer.get_u16_le();
		let height = buffer.get_u16_le();
		if height >= 1 && buffer.remaining() >= ((height - 1) * 4).into() {
			for _ in 0..(height - 1) {
				line_offsets.push(buffer.get_u32_le());
			}
			return Some(ImageHeader {
				width,
				height,
				line_offsets
			});
		}
	}
	return None;
}

fn read_image_data(contents: &[u8], header: &ImageHeader, pixel_format: u32) -> RgbaImage {
	let mut img = RgbaImage::new(header.width as u32, header.height as u32);
	for (y, line_offset) in (&header.line_offsets).iter().enumerate() {
		let mut buffer = Bytes::copy_from_slice(contents);
		buffer.advance(*line_offset as usize);
		let mut x: u16 = 0;
		while x < header.width {
			if buffer.remaining() >= 2 {
				let run_header = buffer.get_u16_le();
				let run_type = run_header & 0x1; // 0 = transparent, 1 = color
				let run_length = (run_header & 0xfffe) >> 1;
				if run_type == 1 && buffer.remaining() >= (run_length * 2).into() {
					for i in 0..run_length {
						let color = read_pixel_data(buffer.get_u16_le(), pixel_format);
						img.put_pixel((x + i) as u32, y as u32, color);
					}
				} else if run_type == 0 {
					for i in 0..run_length {
						img.put_pixel((x + i) as u32, y as u32, Rgba([0, 0, 0, 0]));
					}
				}
				x += run_length;
			}
		}
	}
	return img;
}

fn read_pixel_data(pixel: u16, pixel_format: u32) -> Rgba<u8> {
	match pixel_format {
		2 => {
			// 555 format
			let r = ((pixel & 0x7c00) >> 7) as u8;
			let g = ((pixel & 0x03e0) >> 2) as u8;
			let b = ((pixel & 0x001f) << 3) as u8;
			Rgba([r, g, b, 255])
		},
		_ => {
			// 565 format
			let r = ((pixel & 0xf800) >> 8) as u8;
			let g = ((pixel & 0x07e0) >> 3) as u8;
			let b = ((pixel & 0x001f) << 3) as u8;
			Rgba([r, g, b, 255])
		}
	}
}

pub fn decode(contents: &[u8]) {
	let mut buffer = Bytes::copy_from_slice(contents);
	if let Some(file_header) = read_file_header(&mut buffer) {
		println!("pixel format: {:?}, image count: {:?}", file_header.pixel_format, file_header.image_count);
		let mut image_headers: Vec<ImageHeader> = Vec::new();
		for i in 0..file_header.image_count {
			if let Some(image_header) = read_image_header(&mut buffer) {
				image_headers.push(image_header);
			}
		}
		for (i, image_header) in image_headers.iter().enumerate() {
			println!("IMAGE {:?}: width {:?}, height {:?}", i, image_header.width, image_header.height);
			let img = read_image_data(&contents, &image_header, file_header.pixel_format);
			img.save("test.png");
		}
	}
}
