extern crate glium;

use math::Vector3D;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use std::f64::consts;

#[derive(Copy, Clone)]
pub struct CLight {
	pub pos:    [f32; 3],
	pub color:  [f32; 3],

	pub attenuation: [f32; 3],

	pub vector:     [f32; 3],
	pub range:      f32,
	pub maxradius:  f32,
} 

impl CLight {
	pub fn new() -> CLight {
		CLight{ pos:         [0.0, 0.0, 0.0],
				color:  	 [1.0, 0.0, 0.0],
				attenuation: [0.1, 0.1, 0.03],
				vector: 	 [1.0, 0.0, 0.0], 
				range:       (consts::PI) as f32,
				maxradius:   20.0 }
	}

	pub fn set_pos(&mut self, pos: [f32; 3]) {
		self.pos = pos;
	}
}

#[derive(Copy, Clone)]
pub struct CDirectionLight {
	pub color:  [f32; 3],
	pub vector: [f32; 3],
} 

impl CDirectionLight {
	pub fn new() -> CDirectionLight {
		CDirectionLight{ color:  	 [0.3, 0.3, 0.3],
						 vector: 	 [1.0, 0.5, 0.0],}
	}
}