extern crate glium;
extern crate cgmath;

use math::Vertex;
use math::VertexPT;
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
use glium::index::PrimitiveType;
use std::rc::Rc;

pub struct Render {
	pub pos_texture:   Texture2d,
	pub norm_texture:  Texture2d,
	pub text_texture:  Texture2d,

	pub depthtexture:  DepthTexture2d,
	pub light_texture: Texture2d,

	pub vertex_buffer: glium::VertexBuffer<VertexPT>,
	pub index_buffer:  glium::IndexBuffer<u16>,

	pub orthomatrix:  [[f32; 4]; 4],
}

impl Render {
	pub fn new(display: &GlutinFacade, width: u32, height: u32) -> Render {
		let pos_texture   = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();
    	let norm_texture  = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();
    	let text_texture  = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();
		let light_texture = Texture2d::empty_with_format(display, F32F32F32F32, NoMipmap, width, height).unwrap();

    	let depthtexture  = DepthTexture2d::empty_with_format(display, DepthFormat::F32, NoMipmap, width, height).unwrap();

    	let Verteces = [
            VertexPT{ position: [0.0,          0.0,           0.0], tex_coord: [0.0, 0.0] },
            VertexPT{ position: [width as f32, 0.0,           0.0], tex_coord: [1.0, 0.0] },
            VertexPT{ position: [width as f32, height as f32, 0.0], tex_coord: [1.0, 1.0] },
            VertexPT{ position: [0.0,          height as f32, 0.0], tex_coord: [0.0, 1.0] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &Verteces).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[1, 0, 2, 0, 2, 3u16]
            ).unwrap();

        let ortho_matrix: cgmath::Matrix4<f32> = cgmath::ortho(0.0, 800.0, 0.0, 600.0, -1.0, 1.0);
        let orthomatrix = Into::<[[f32; 4]; 4]>::into(ortho_matrix);

    	Render { 
    		pos_texture:   pos_texture,
			norm_texture:  norm_texture,
			text_texture:  text_texture,
			light_texture: light_texture,

	 	 	depthtexture:  depthtexture,

	 	 	vertex_buffer: vertex_buffer,
	 	 	index_buffer:  index_buffer,
	 	 	orthomatrix:   orthomatrix,
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