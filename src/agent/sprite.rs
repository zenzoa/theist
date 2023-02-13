use crate::image_format::{ c16, s16, blk };
use super::file::FileType;
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
		filetype: FileType,
		output_filename: String,
		input_filename: String,
		data: Option<Bytes>
	},
	Png {
		filetype: FileType,
		output_filename: String,
		input_filename: String,
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
					filetype: FileType::Sprite,
					output_filename,
					input_filename: input_filename.replace(".c16", ".png"),
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
					filetype: FileType::Sprite,
					output_filename,
					input_filename: input_filename.replace(".s16", ".png"),
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
					filetype: FileType::Sprite,
					output_filename,
					input_filename: input_filename.replace(".blk", ".png"),
					frames: vec![ SpriteFrame::new_from_data(&png_filename, &mut Bytes::from(blk_data.into_inner()))? ],
					data: Some(data.clone())
				})
			},
			"png" => {
				let mut new_sprite = Sprite::Png{
					filetype: FileType::Sprite,
					output_filename: format!("{}.c16", title),
					input_filename: input_filename.to_string(),
					frames: vec![ SpriteFrame::new_from_data(input_filename, data)? ],
					data: None
				};
				new_sprite.fetch_data(&"".to_string())?;
				Ok(new_sprite)
			},
			_ => {
				Err(create_error(format!("Image {} is not a valid creatures image file.", &input_filename).as_str()))
			}
		}
	}

	pub fn new_from_data_raw(input_filename: &String, data: &mut Bytes) -> Result<Sprite, Box<dyn Error>> {
		Ok(Sprite::Raw{
			filetype: FileType::Sprite,
			output_filename: file_helper::filename(input_filename),
			input_filename: input_filename.to_string(),
			data: Some(data.clone())
		})
	}

	pub fn add_frame(&mut self, new_frame: SpriteFrame) -> bool {
		if let Sprite::Png{ output_filename, frames, .. } = self {
			if file_helper::extension(output_filename) != "blk" || frames.is_empty() {
				let mut frame_already_in_list = false;
				for frame in frames.iter() {
					if frame.input_filename == new_frame.input_filename {
						frame_already_in_list = true;
					}
				}
				if !frame_already_in_list {
					frames.push(new_frame);
					return true;
				}
			}
		}
		false
	}

	pub fn remove_frame(&mut self, index: usize) -> bool {
		if let Sprite::Png{ frames, .. } = self {
			if index < frames.len() {
				frames.remove(index);
				return true;
			}
		}
		false
	}

	pub fn move_frame_up(&mut self, index: usize) -> bool {
		if let Sprite::Png{ frames, .. } = self {
			if index > 0 && index < frames.len() {
				frames.swap(index, index - 1);
				return true;
			}
		}
		false
	}

	pub fn move_frame_down(&mut self, index: usize) -> bool {
		if let Sprite::Png{ frames, .. } = self {
			if index + 1 < frames.len() {
				frames.swap(index, index + 1);
				return true;
			}
		}
		false
	}

	pub fn get_output_filename(&self) -> String {
		match self {
			Sprite::Raw{ output_filename, .. } => output_filename.to_string(),
			Sprite::Png{ output_filename, .. } => output_filename.to_string()
		}
	}

	pub fn fetch_data(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
		match self {
			Sprite::Raw{ input_filename, data, .. } => {
				let contents = fs::read(format!("{}{}", path, input_filename))?;
				*data = Some(Bytes::copy_from_slice(&contents));
				Ok(())
			},
			Sprite::Png{ output_filename, frames, data, .. } => {
				let mut images: Vec<RgbaImage> = Vec::new();
				for frame in frames {
					let image_data = match &frame.data {
						Some(bytes) => ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?,
						None => ImageReader::open(format!("{}{}", path, frame.input_filename))?.decode()?
					};
					images.push(image_data.into_rgba8());
				}
				*data = match file_helper::extension(output_filename).as_str() {
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
