extern crate glium;

use models;
use math::Vertex;
use texture::CTexture;
use program::CProgram;
use math::Vector3D;
use math::Matrix4D;
use camera::CCamera;
use light::CLight;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface};
use glium::framebuffer::MultiOutputFrameBuffer;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use glium::texture::DepthTexture2d;
use glium::texture::UncompressedFloatFormat::F32F32F32F32;
use glium::texture::DepthFormat;
use glium::texture::MipmapsOption::NoMipmap;
use std::io;
use std::rc::Rc;
use std::io::prelude::*;
use std::fs::File;

pub struct Render<'a> {
	pub pos_texture:       &'a Texture2d,
	pub norm_texture:      &'a Texture2d,
	pub text_texture:      &'a Texture2d,

	pub gbuffer:           MultiOutputFrameBuffer<'a>,
	pub light_buffer:       SimpleFrameBuffer<'a>,

	pub depthtexture:      &'a DepthTexture2d,
	pub light_texture:     &'a Texture2d,
}

pub struct CRenderObject {
	pub vertex_buffer: glium::VertexBuffer<Vertex>,
	pub index_buffer:  glium::index::IndexBuffer<u32>,

	pub texture:  Rc<CTexture>,
	pub program:  Rc<CProgram>
}

impl CRenderObject {
	pub fn new(display: &GlutinFacade, mdl: &models::CModel, texture: &Rc<CTexture>, program: &Rc<CProgram>) -> CRenderObject {
		let new_vertex_buffer = glium::VertexBuffer::new(display, &mdl.vertcs).unwrap();
    	let new_index_buffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &mdl.inds).unwrap();


   	 	CRenderObject{ vertex_buffer: new_vertex_buffer, 
   	 				   index_buffer:  new_index_buffer, 
   	 			       program:       program.clone(), 
   	 			       texture:       texture.clone(), }
	}      

	pub fn draw(&self, mut target: &mut MultiOutputFrameBuffer, PM: &Matrix4D, VM: &Matrix4D, MM: &Matrix4D) {
		let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };

		let uniforms = uniform! {
	        perspective_matrix: (*PM).matrix,
	        view_matrix: (*VM).matrix,
	        model_matrix: (*MM).matrix,
	        tex: &self.texture.texObject,
	    };
	    		
	    target.draw(&self.vertex_buffer, &self.index_buffer, self.program.prog_object(), &uniforms, &params).unwrap();
	}
}