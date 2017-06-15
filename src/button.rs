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
use gui::ControllEvent;
use gui::Controller;
use render::Render;

pub struct Button {
	pub rect: Rect,	
	pub is_taped: bool,
	pub taped_color:  Vector3D,
	pub untaped_color: Vector3D,
	
	eventsPool: Vec<ControllEvent>
}


impl Button {
	pub fn new(prog: &Rc<CProgram>, x: f32, y: f32, width: f32, height: f32) -> Button {
		Button {
			rect: Rect::new(prog, x, y, width, height),
			is_taped: false,
			taped_color: Vector3D::new(1.0, 0.0, 0.0),
			untaped_color: Vector3D::new(0.0, 1.0, 0.0),
			eventsPool: vec![],
		} 
	}

	

	pub fn set_taped_color(&mut self, new_color: Vector3D) {
		self.taped_color = new_color;
	}

	pub fn set_untaped_color(&mut self, new_color: Vector3D) {
		self.untaped_color = new_color;
	}
}


impl Controller for Button {
	fn tap(&mut self, x: f32, y: f32) {
		if self.rect.is_inside(x, y) {
			self.is_taped = true;
			self.eventsPool.push(ControllEvent::Click);
		}
	}

	fn untap(&mut self) {
		self.is_taped = false;
	}

	fn moveTo(&mut self, x: f32, y: f32) { }

	fn get_events(&mut self) -> Vec<ControllEvent> {
		let tmp = self.eventsPool.clone();
		self.eventsPool.clear();
		tmp
	}

	fn draw(&mut self, display: &GlutinFacade, render: &mut Render, canvas: &mut glium::Frame) {
		if self.is_taped {
			self.rect.color = self.taped_color;
		} 
		else {
			self.rect.color = self.untaped_color;
		}
		self.rect.draw(display, render, canvas);
	}

	fn setValue(&mut self, value: f32) { }
	fn getValue(&mut self) -> f32 {	0.0 }
}