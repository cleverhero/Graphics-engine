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

pub struct TrackBar {
	pub backrect: Rect,	
	pub slider: Rect,
	pub is_taped: bool,
	pub value: f32,
	pub minValue: f32,
	pub maxValue: f32,
	
	eventsPool: Vec<ControllEvent>
}


impl TrackBar {
	pub fn new(prog: &Rc<CProgram>, x: f32, y: f32, width: f32, height: f32)  -> TrackBar {
		let mut backrect = Rect::new(prog, x, y, width, height);
		backrect.color = Vector3D::new(0.0, 0.0, 0.0);

		let mut slider = Rect::new(prog, x, y, width*0.1, height);
		slider.color = Vector3D::new(0.7, 0.7, 0.7);

		TrackBar {
			value: 0.0,
			backrect: backrect,
			slider: slider,
			is_taped: false,
			eventsPool: vec![],
			minValue: 0.0,
			maxValue: 100.0
		} 
	}
}


impl Controller for TrackBar {
	fn tap(&mut self, x: f32, y: f32) {
		if self.slider.is_inside(x, y) {
			self.is_taped = true;
			self.eventsPool.push(ControllEvent::Click);
		}
	}

	fn untap(&mut self) {
		self.is_taped = false;
	}

	fn moveTo(&mut self, x: f32, y: f32) { 
		if (!self.is_taped) { return; }
		self.slider.x = x;
		if (x < self.backrect.x) { self.slider.x = self.backrect.x }
		if (x > self.backrect.x + self.backrect.width - self.slider.width) { self.slider.x = self.backrect.x + self.backrect.width - self.slider.width }

		self.value = self.minValue + ((self.slider.x - self.backrect.x) / (self.backrect.width - self.slider.width)) * ( self.maxValue -  self.minValue );
		self.eventsPool.push(ControllEvent::Changed);
	}

	fn get_events(&mut self) -> Vec<ControllEvent> {
		let tmp = self.eventsPool.clone();
		self.eventsPool.clear();
		tmp
	}

	fn draw(&mut self, display: &GlutinFacade, render: &mut Render, canvas: &mut glium::Frame) {
		self.slider.x = self.backrect.x + ((self.value - self.minValue) / (self.maxValue -  self.minValue))*(self.backrect.width - self.slider.width);

		self.backrect.draw(display, render, canvas);
		self.slider.draw(display, render, canvas);
	}

	fn setValue(&mut self, value: f32) {
		self.value = value;
	}

	fn getValue(&mut self) -> f32 {
		self.value
	}
}
