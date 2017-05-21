extern crate cgmath;
extern crate glium;

use program::CProgram;
use glium::index::PrimitiveType;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use math::VertexPT;
use math::Size2;
use math::Vector3D;
use std::rc::Rc;

pub struct Rect {
	prog: Rc<CProgram>,
	color: Vector3D,

	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}



impl Rect {
	pub fn new(prog: &Rc<CProgram>, x: f32, y: f32, width: f32, height: f32) -> Rect {
		Rect {
			x: x,
			y: y,

			width:  width,
			height: height,

			prog: prog.clone(),

			color: Vector3D::new(0.3, 0.3, 0.3),
		}
	}

	pub fn set_color(&mut self, new_color: Vector3D) {
		self.color = new_color;
	}

	pub fn draw(&self, display: &GlutinFacade, canvas: &mut glium::Frame, orthomatrix: &[[f32; 4]; 4]) {
		let x0 = self.x;
		let y0 = self.y;
		let x1 = self.x + self.width;
		let y1 = self.y + self.height;

		let Verteces = [
			VertexPT{ position: [x0, y0, 0.0], tex_coord: [0.0, 0.0] },
        	VertexPT{ position: [x1, y0, 0.0], tex_coord: [1.0, 0.0] },
        	VertexPT{ position: [x1, y1, 0.0], tex_coord: [1.0, 1.0] },
        	VertexPT{ position: [x0, y1, 0.0], tex_coord: [0.0, 1.0] },
		];

		let quad_vertex_buffer = glium::VertexBuffer::new(display, &Verteces).unwrap();
    	let quad_index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[1, 0, 2, 0, 2, 3u16]
    		).unwrap();

    	let color = Vector3D::new(0.3, 0.3, 0.3);

    	let uniforms = uniform! {
            matrix: *orthomatrix,
            color:  self.color.as_arr(),
        };

		canvas.draw(&quad_vertex_buffer, &quad_index_buffer, self.prog.prog_object(), &uniforms, &Default::default()).unwrap();
	}
}

pub struct Botton {
	pub rect: Rect,	
	
}

impl Botton {
	pub fn new(prog: &Rc<CProgram>, x: f32, y: f32, width: f32, height: f32) -> Botton {
		Botton {
			rect: Rect::new(prog, x, y, width, height)
		} 
	}

	pub fn draw(&self, display: &GlutinFacade, canvas: &mut glium::Frame, orthomatrix: &[[f32; 4]; 4]) {
		self.rect.draw(display, canvas, orthomatrix);
	}

	pub fn set_color(&mut self, new_color: Vector3D) {
		self.rect.color = new_color;
	}
}

pub struct Interface {
	pub bottons:  Vec<Botton>,

	orthomatrix: [[f32; 4]; 4],
}

impl Interface {
	pub fn new(display: &GlutinFacade, winSize: Size2) -> Interface {
		let orthomatrix: cgmath::Matrix4<f32> = cgmath::ortho(0.0, winSize.w as f32, 0.0, winSize.h as f32, -1.0, 1.0);
    	let orthomatrix = Into::<[[f32; 4]; 4]>::into(orthomatrix);

    	let prog = Rc::new(CProgram::load(display, "Shaders/2DV.vs", "Shaders/2DF.fs"));

    	let botton1 = Botton::new(&prog, 10.0, 10.0, 100.0, 100.0);

    	let mut botton2 = Botton::new(&prog, 500.0, 500.0, 800.0, 600.0);
    	botton2.set_color(Vector3D::new(0.9, 0.0, 0.0));

    	Interface {
    		bottons: vec![ botton1, botton2 ],
    		orthomatrix: orthomatrix
    	}
	}

	pub fn draw(&self, display: &GlutinFacade, mut canvas: &mut glium::Frame) {
		for botton in &self.bottons {
	    	botton.draw(display, canvas, &self.orthomatrix);
	    }
	}
}
