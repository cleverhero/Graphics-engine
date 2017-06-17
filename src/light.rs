extern crate glium;

use math::Vector3D;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use std::f64::consts;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use program::CProgram;
use std::rc::Rc;

use std::f32;
use std::str::FromStr;

pub struct CLight {
	pub pos:    Vector3D,
	pub color:  Vector3D,

	pub attenuation: Vector3D,

	pub vector:     Vector3D,
	pub range:      f32,
	pub maxradius:  f32,

} 

impl CLight {
	pub fn new() -> CLight {
		CLight{ pos:         Vector3D::new(0.0, 0.0, 0.0),
				color:  	 Vector3D::new(1.0, 0.0, 0.0),
				attenuation: Vector3D::new(0.1, 0.1, 0.03),
				vector: 	 Vector3D::new(1.0, 0.0, 0.0), 
				range:       (consts::PI) as f32,
				maxradius:   20.0,}
	}

	pub fn set_pos(&mut self, pos: Vector3D) {
		self.pos = pos;
	}

	pub fn set_color(&mut self, color: Vector3D) {
		self.color = color;
	}

	pub fn set_range(&mut self, range: f32) {
		self.range = range;
	}

	pub fn set_vector(&mut self, vector: Vector3D) {
		self.vector = vector;
	}

	pub fn save(&self) -> String {
		self.pos.to_string() + &" " + &self.color.to_string() + &" " + &self.vector.to_string() + &" " + &self.range.to_string()
	}

	pub fn load(data: String) -> CLight {
		let items: Vec<&str> = data.split(" ").collect();
		let pos = Vector3D::new(
			f32::from_str(items[0]).unwrap(),
			f32::from_str(items[1]).unwrap(),
			f32::from_str(items[2]).unwrap()
		);
		let color = Vector3D::new(
			f32::from_str(items[3]).unwrap(),
			f32::from_str(items[4]).unwrap(),
			f32::from_str(items[5]).unwrap()
		);
		let vector = Vector3D::new(
			f32::from_str(items[6]).unwrap(),
			f32::from_str(items[7]).unwrap(),
			f32::from_str(items[8]).unwrap()
		);
		let range = f32::from_str(items[9]).unwrap();

		let mut new_light = CLight::new();
		new_light.set_pos(pos);
		new_light.set_color(color);
		new_light.set_vector(vector);
		new_light.set_range(range);

		new_light
	}
}

pub struct CDirectionLight {
	pub color:  Vector3D,
	pub vector: Vector3D,
} 

impl CDirectionLight {
	pub fn new() -> CDirectionLight {
		CDirectionLight{ color:  Vector3D::new(0.3, 0.3, 0.3),
						 vector: Vector3D::new(1.0, 0.5, 0.0), }
	}

	pub fn set_color(&mut self, color: Vector3D) {
		self.color = color;
	}

	pub fn set_vector(&mut self, vector: Vector3D) {
		self.vector = vector;
	}

	pub fn save(&self) -> String {
		self.color.to_string() + &" " + &self.vector.to_string()
	}

	pub fn load(data: String) -> CDirectionLight {
		let items: Vec<&str> = data.split(" ").collect();;
		let color = Vector3D::new(
			f32::from_str(items[0]).unwrap(),
			f32::from_str(items[1]).unwrap(),
			f32::from_str(items[2]).unwrap()
		);
		let vector = Vector3D::new(
			f32::from_str(items[3]).unwrap(),
			f32::from_str(items[4]).unwrap(),
			f32::from_str(items[5]).unwrap()
		);

		let mut new_light = CDirectionLight::new();
		new_light.set_color(color);
		new_light.set_vector(vector);

		new_light
	}
}