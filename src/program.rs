use glium;
use glium::{DisplayBuild, Surface, glutin, texture};
use glium::backend::glutin_backend::GlutinFacade;
use std::fs::File;
use std::io::Cursor;
use std::io;
use std::io::prelude::*;

pub struct CProgram {
	pub prog_object: glium::Program,
}

impl CProgram {
	pub fn load(display: &GlutinFacade, VS_srs: &str, FS_srs: &str) -> CProgram {
		let mut f = match File::open(VS_srs) {
			Ok(f) => f,
        	Err(err) => panic!("file error: {}", err)
		};
		let mut vertex_shader_src = String::new();
		f.read_to_string(&mut vertex_shader_src);
		
		let mut f = match File::open(FS_srs) {
			Ok(f) => f,
   	     	Err(err) => panic!("file error: {}", err)
		};
		let mut fragment_shader_src = String::new();
		f.read_to_string(&mut fragment_shader_src);
		
   	 	let new_program = glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

   	 	CProgram { prog_object: new_program }
	}

	pub fn prog_object(&self) -> &glium::Program {
		&self.prog_object
	}
}