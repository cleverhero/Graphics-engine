use glium;
use glium::{DisplayBuild, Surface, glutin, texture};
use glium::backend::glutin_backend::GlutinFacade;
use image;
use std::fs::File;
use std::io::Cursor;
use std::io;
use std::io::prelude::*;

pub struct CTexture {
	pub texObject: texture::SrgbTexture2d,
}

impl CTexture {
	pub fn load(display: &GlutinFacade, filepath: &str) -> CTexture {
		let mut f = File::open(filepath).unwrap();
		let mut buffer = Vec::new();

		f.read_to_end(&mut buffer);

		let image = image::load(Cursor::new(&buffer[..]),
                            image::JPEG).unwrap().to_rgba();
    	let image_dimensions = image.dimensions();
    	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    	let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
    	CTexture{ texObject: texture }
	}

	pub fn getTextureObject(&self) -> &texture::SrgbTexture2d {
		&self.texObject
	}
}