use crate::image_format::{ c16, s16, blk };
use crate::error::create_error;
use crate::file_helper;

use std::fs;
use std::io::Cursor;
use std::error::Error;
use bytes::Bytes;
use image::io::Reader as ImageReader;
use image::{ RgbaImage, ImageOutputFormat };

#[derive(Clone)]
pub enum Sprite {
	Raw {
		output_filename: String,
		input_filename: String,
		data: Option<Bytes>
	},
	Png {
		output_filename: String,
		frames: Vec<SpriteFrame>, // Blk's only care about first frame
		data: Option<Bytes>
	}
}

#[derive(Clone)]
pub struct SpriteFrame {
	pub input_filename: String,
	pub data: Option<Bytes>
}

impl Sprite {
	// pub fn new(input_filename: &String) -> Result<Sprite, Box<dyn Error>> {
	// 	let output_filename = file_helper::filename(input_filename);
	// 	match file_helper::extension(input_filename).as_str() {
	// 		"c16" => {
	// 			Ok(Sprite::Raw{
	// 				output_filename,
	// 				input_filename: input_filename.to_string(),
	// 				data: None
	// 			})
	// 		},
	// 		"s16" => {
	// 			Ok(Sprite::Raw{
	// 				output_filename,
	// 				input_filename: input_filename.to_string(),
	// 				data: None
	// 			})
	// 		},
	// 		"blk" => {
	// 			Ok(Sprite::Raw{
	// 				output_filename,
	// 				input_filename: input_filename.to_string(),
	// 				data: None
	// 			})
	// 		},
	// 		"png" => {
	// 			Ok(Sprite::Png{
	// 				output_filename,
	// 				frames: vec![ SpriteFrame::new(input_filename)? ],
	// 				data: None
	// 			})
	// 		},
	// 		_ => {
	// 			Err(create_error("Unrecognized file type. Sprite must be a C16, S16, BLK, or PNG."))
	// 		}
	// 	}
	// }

	pub fn new_from_data(input_filename: &String, data: &mut Bytes) -> Result<Sprite, Box<dyn Error>> {
		let output_filename = file_helper::filename(input_filename);
		let title = file_helper::title(input_filename);
		match file_helper::extension(input_filename).as_str() {
			"c16" => {
				let images = c16::decode(data)?;
				let mut frames = Vec::new();
				for (i, image) in images.iter().enumerate() {
					let png_filename = format!("{}-{}.png", title, i + 1);
					let mut png_data = Cursor::new(Vec::new());
					image.write_to(&mut png_data, ImageOutputFormat::Png)?;
					frames.push(SpriteFrame::new_from_data(&png_filename, &mut Bytes::from(png_data.into_inner()))?);
				}
				Ok(Sprite::Png{
					output_filename,
					frames,
					data: Some(data.clone())
				})
			},
			"s16" => {
				let images = s16::decode(data)?;
				let mut frames = Vec::new();
				for (i, image) in images.iter().enumerate() {
					let png_filename = format!("{}-{}.png", title, i + 1);
					let mut png_data = Cursor::new(Vec::new());
					image.write_to(&mut png_data, ImageOutputFormat::Png)?;
					frames.push(SpriteFrame::new_from_data(&png_filename, &mut Bytes::from(png_data.into_inner()))?);
				}
				Ok(Sprite::Png{
					output_filename,
					frames,
					data: Some(data.clone())
				})
			},
			"blk" => {
				let image = blk::decode(data)?;
				let mut blk_data = Cursor::new(Vec::new());
				image.write_to(&mut blk_data, ImageOutputFormat::Png)?;
				let png_filename = format!("{}.png", title);
				Ok(Sprite::Png{
					output_filename,
					frames: vec![ SpriteFrame::new_from_data(&png_filename, &mut Bytes::from(blk_data.into_inner()))? ],
					data: Some(data.clone())
				})
			},
			_ => {
				Err(create_error(format!("Image {} is not a valid creatures image file.", &input_filename).as_str()))
			}
		}
	}

	pub fn add_frame(&mut self, new_frame: SpriteFrame) {
		if let Sprite::Png{ frames, .. } = self {
			let mut frame_already_in_list = false;
			for frame in frames.iter() {
				if frame.input_filename == new_frame.input_filename {
					frame_already_in_list = true;
				}
			}
			if !frame_already_in_list {
				frames.push(new_frame);
			}
		}
	}

	// pub fn remove_frame(&mut self, index: usize) {
	// 	if let Sprite::Png{ frames, .. } = self {
	// 		if index < frames.len() {
	// 			frames.remove(index);
	// 		}
	// 	}
	// }

	// pub fn move_frame_up(&mut self, index: usize) {
	// 	if let Sprite::Png{ frames, .. } = self {
	// 		if index > 0 && index < frames.len() {
	// 			frames.swap(index, index - 1);
	// 		}
	// 	}
	// }

	// pub fn move_frame_down(&mut self, index: usize) {
	// 	if let Sprite::Png{ frames, .. } = self {
	// 		if index + 1 < frames.len() {
	// 			frames.swap(index, index + 1);
	// 		}
	// 	}
	// }

	pub fn get_output_filename(&self) -> String {
		match self {
			Sprite::Raw{ output_filename, .. } => output_filename.to_string(),
			Sprite::Png{ output_filename, .. } => output_filename.to_string()
		}
	}

	pub fn get_title(&self) -> String {
		file_helper::title(&self.get_output_filename())
	}

	pub fn get_extension(&self) -> String {
		file_helper::extension(&self.get_output_filename())
	}

	pub fn get_data(&self) -> Option<Bytes> {
		match self {
			Sprite::Raw{ data, .. } => data.clone(),
			Sprite::Png{ data, .. } => data.clone()
		}
	}

	pub fn fetch_data(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
		let extension = self.get_extension();
		match self {
			Sprite::Raw{ input_filename, data, .. } => {
				let contents = fs::read(format!("{}{}", path, input_filename))?;
				*data = Some(Bytes::copy_from_slice(&contents));
				Ok(())
			},
			Sprite::Png{ frames, data, .. } => {
				let mut images: Vec<RgbaImage> = Vec::new();
				for frame in frames {
					let image_data = ImageReader::open(format!("{}{}", path, frame.input_filename))?;
					let image = image_data.decode()?;
					images.push(image.into_rgba8());
				}
				*data = match extension.as_str() {
					"c16" => Some(c16::encode(images)),
					"s16" => Some(s16::encode(images)),
					"blk" => Some(if let Some(image) = images.get(0) { blk::encode(image.clone()) } else { Bytes::new() }),
					_ => data.clone()
				};
				Ok(())
			}
		}
	}
}

impl SpriteFrame {
	// pub fn new(input_filename: &String) -> Result<SpriteFrame, Box<dyn Error>> {
	// 	if file_helper::extension(input_filename) == "png" {
	// 		Ok(SpriteFrame{
	// 			input_filename: input_filename.to_string(),
	// 			data: None
	// 		})
	// 	} else {
	// 		Err(create_error(&format!("File {} has an unrecognized file type. Sprite frame must be a PNG.", input_filename)))
	// 	}
	// }

	pub fn new_from_data(input_filename: &String, data: &mut Bytes) -> Result<SpriteFrame, Box<dyn Error>> {
		if file_helper::extension(input_filename) == "png" {
			Ok(SpriteFrame{
				input_filename: input_filename.to_string(),
				data: Some(data.clone())
			})
		} else {
			Err(create_error(&format!("File {} has an unrecognized file type. Sprite frame must be a PNG.", input_filename)))
		}
	}
}
