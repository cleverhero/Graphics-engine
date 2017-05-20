extern crate glium;

use math::Vector3D;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use std::f64::consts;

#[derive(Copy, Clone)]
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
				maxradius:   20.0 }
	}

	pub fn set_pos(&mut self, pos: Vector3D) {
		self.pos = pos;
	}
}

#[derive(Copy, Clone)]
pub struct CDirectionLight {
	pub color:  Vector3D,
	pub vector:     Vector3D,
} 

impl CDirectionLight {
	pub fn new() -> CDirectionLight {
		CDirectionLight{ color:  	 Vector3D::new(0.3, 0.3, 0.3),
						 vector: 	 Vector3D::new(1.0, 0.5, 0.0),}
	}
}