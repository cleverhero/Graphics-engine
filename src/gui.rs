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

#[derive(Debug, Clone, Copy)]
enum ControllEvent {
	Click
}

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

pub struct Button {
	pub rect: Rect,	
	pub is_taped: bool,
	pub taped_color:  Vector3D,
	pub untaped_color: Vector3D,
	
	eventsPool: Vec<ControllEvent>
}


impl Button {
	pub fn new(prog: &Rc<CProgram>, id: i32, x: f32, y: f32, width: f32, height: f32) -> Button {
		Button {
			rect: Rect::new(prog, x, y, width, height),
			is_taped: false,
			taped_color: Vector3D::new(1.0, 0.0, 0.0),
			untaped_color: Vector3D::new(0.0, 1.0, 0.0),
			eventsPool: vec![],
		} 
	}

	pub fn draw(&mut self, display: &GlutinFacade, canvas: &mut glium::Frame, orthomatrix: &[[f32; 4]; 4]) {
		if self.is_taped {
			self.rect.color = self.taped_color;
		} 
		else {
			self.rect.color = self.untaped_color;
		}
		self.rect.draw(display, canvas, orthomatrix);
	}

	pub fn set_taped_color(&mut self, new_color: Vector3D) {
		self.taped_color = new_color;
	}

	pub fn set_untaped_color(&mut self, new_color: Vector3D) {
		self.untaped_color = new_color;
	}

	pub fn tap(&mut self, x: f32, y: f32) {
		if self.rect.is_inside(x, y) {
			self.is_taped = true;
			self.eventsPool.push(ControllEvent::Click);
		}
	}

	pub fn untap(&mut self) {
		self.is_taped = false;
	}

	pub fn get_events(&mut self) -> Vec<ControllEvent> {
		let tmp = self.eventsPool.clone();
		self.eventsPool.clear();
		tmp
	}
}

pub struct Interface {
	pub buttons:  Vec<Button>,
	nextElementId: i32,

	cursor:      Point,
	orthomatrix: [[f32; 4]; 4],

	flag: bool,
}

impl Interface {
	pub fn new(display: &GlutinFacade, winSize: Size2) -> Interface {
		let orthomatrix: cgmath::Matrix4<f32> = cgmath::ortho(0.0, winSize.w as f32, 0.0, winSize.h as f32, -1.0, 1.0);
    	let orthomatrix = Into::<[[f32; 4]; 4]>::into(orthomatrix);

    	let prog = Rc::new(CProgram::load(display, "Shaders/2DV.vs", "Shaders/2DF.fs"));

    	let mut botton1 = Button::new(&prog, 0, 10.0, 570.0, 60.0, 20.0);

    	Interface {
    		buttons: vec![ botton1 ],
    		orthomatrix: orthomatrix,
    		cursor:      Point::new(0.0, 0.0),
    		flag:        true,
    		nextElementId: 1,
    	}
	}

	pub fn draw(&mut self, display: &GlutinFacade, mut canvas: &mut glium::Frame) {
		for button in &mut self.buttons {
	    	button.draw(display, canvas, &self.orthomatrix);
	    }
	}

	pub fn checkEvents(&mut self, event: &glium::glutin::Event, display: &GlutinFacade) {
		match *event {
			glutin::Event::MouseMoved(x, y) => {
				self.cursor.x = x as f32;
				self.cursor.y = 600.0 - y as f32;
			}
            glutin::Event::MouseInput(state, glutin::MouseButton::Left) => {
				for button in &mut self.buttons {
					match state {
						Pressed => {
							button.tap(self.cursor.x, self.cursor.y);
						},
						Release => {
							button.untap();
						},
					}
				}
            },
            _ => {}
        }
	}

	pub fn update(&mut self) {
		for event in self.buttons[0].get_events() {
			self.OnEvent();
		}
	}

	pub fn OnEvent(&mut self) {
		self.flag = !self.flag;
		println!("{}", self.flag);
	}
}
