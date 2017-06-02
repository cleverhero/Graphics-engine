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
use trackbar::TrackBar;


#[derive(Debug, Clone, Copy)]
pub enum ControllEvent {
	Click,
	Changed,
}

pub trait Controller {
	fn tap(&mut self, x: f32, y: f32);
	fn untap(&mut self);
	fn moveTo(&mut self, x: f32, y: f32);

	fn get_events(&mut self) -> Vec<ControllEvent>;

	fn draw(&mut self, display: &GlutinFacade, canvas: &mut glium::Frame, orthomatrix: &[[f32; 4]; 4]);
}

pub struct Interface {
	pub elements:  Vec<Box<Controller>>,

	cursor:      Point,
	orthomatrix: [[f32; 4]; 4],

	flag: bool,
}

impl Interface {
	pub fn new(display: &GlutinFacade, winSize: Size2) -> Interface {
		let orthomatrix: cgmath::Matrix4<f32> = cgmath::ortho(0.0, winSize.w as f32, 0.0, winSize.h as f32, -1.0, 1.0);
    	let orthomatrix = Into::<[[f32; 4]; 4]>::into(orthomatrix);

    	let prog = Rc::new(CProgram::load(display, "Shaders/2DV.vs", "Shaders/2DF.fs"));

    	let mut botton1 = Box::new( Button::new(&prog, 10.0, 570.0, 60.0, 20.0) );
    	let mut trackBar1 = Box::new( TrackBar::new(&prog, 10.0, 370.0, 60.0, 20.0) );

    	Interface {
    		elements: vec![ botton1, trackBar1 ],
    		orthomatrix: orthomatrix,
    		cursor:      Point::new(0.0, 0.0),
    		flag:        true,
    	}
	}

	pub fn draw(&mut self, display: &GlutinFacade, mut canvas: &mut glium::Frame) {
		for element in &mut self.elements {
	    	element.draw(display, canvas, &self.orthomatrix);
	    }
	}

	pub fn checkEvents(&mut self, event: &glium::glutin::Event, display: &GlutinFacade) {
		match *event {
			glutin::Event::MouseMoved(x, y) => {
				self.cursor.x = x as f32;
				self.cursor.y = 600.0 - y as f32;
				for element in &mut self.elements {
					element.moveTo(self.cursor.x, self.cursor.y);
				}
			}
            glutin::Event::MouseInput(state, glutin::MouseButton::Left) => {
				for element in &mut self.elements {
					match state {
						Pressed => {
							element.tap(self.cursor.x, self.cursor.y);
						},
						Release => {
							element.untap();
						},
					}
				}
            },
            _ => {}
        }
	}

	pub fn update(&mut self) {
		for event in self.elements[0].get_events() {
			match event {
				ControllEvent::Click => { self.OnEvent(); },
				_ => { }
			}
		}
	}

	pub fn OnEvent(&mut self) {
		self.flag = !self.flag;
		println!("{}", self.flag);
	}
}
