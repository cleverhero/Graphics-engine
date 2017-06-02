extern crate cgmath;
extern crate glium;

use program::CProgram;
use glium::index::PrimitiveType;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use glutin::ElementState::Pressed;
use glutin::ElementState::Released;
use math::VertexPT;
use math::Size2;
use math::Point;
use math::Vector3D;
use std::rc::Rc;

pub struct Rect {
	prog: Rc<CProgram>,
	pub color: Vector3D,

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

	pub fn is_inside(&self, x: f32, y: f32) -> bool {
		let x0 = self.x;
		let y0 = self.y;
		let x1 = self.x + self.width;
		let y1 = self.y + self.height;

		if (x >= x0 && x <= x1 && y >= y0 && y <= y1) {
			return true;
		}

		return false;
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