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
use models2D::Rect;
use button::Button;


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
