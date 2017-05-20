extern crate glium;

use math::Vertex;
use math::Vector3D;
use math::Matrix4D;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface};
use glium::framebuffer::MultiOutputFrameBuffer;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use glium::texture::DepthTexture2d;
use glium::texture::UncompressedFloatFormat::F32F32F32F32;
use glium::texture::DepthFormat;
use glium::texture::MipmapsOption::NoMipmap;
use std::rc::Rc;


pub struct Render {
	pub pos_texture:   Texture2d,
	pub norm_texture:  Texture2d,
	pub text_texture:  Texture2d,

	pub depthtexture:  DepthTexture2d,
	pub light_texture: Texture2d,
}

impl Render {
	pub fn new(display: &GlutinFacade, width: u32, height: u32) -> Render {
		let pos_texture   = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();
    	let norm_texture  = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();
    	let text_texture  = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();
		let light_texture = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();

    	let depthtexture  = DepthTexture2d::empty_with_format(display, DepthFormat::F32, NoMipmap, width, height).unwrap();

    	Render { 
    		pos_texture:   pos_texture,
			norm_texture:  norm_texture,
			text_texture:  text_texture,
			light_texture: light_texture,

	 	 	depthtexture:  depthtexture,
		}
	}

	pub fn get_gbuffer(&self, display: &GlutinFacade) -> MultiOutputFrameBuffer {
		let output = &[("pos_texture",  &self.pos_texture), 
				       ("norm_texture", &self.norm_texture), 
				       ("text_texture", &self.text_texture)];
		MultiOutputFrameBuffer::with_depth_buffer(display, output.iter().cloned(), &self.depthtexture).unwrap()
	} 

	pub fn get_lightbuffer(&self, display: &GlutinFacade) -> SimpleFrameBuffer  {
		SimpleFrameBuffer::with_depth_buffer(display, &self.light_texture, &self.depthtexture).unwrap()
	} 
}