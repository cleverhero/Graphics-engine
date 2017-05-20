extern crate glium;

use models;
use texture::CTexture;
use program::CProgram;
use camera::CCamera;
use light::CLight;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface};
use std::rc::Rc;
use camera::CanBeCamera;
use glium::glutin;

use math::Vector3D;
use math::Matrix4D;
use physical_object::CPhysicalObject;
use render_object::CRenderObject;
use geometry::inters;
use std::cell::Cell;

pub struct CViewer {
	pub pos: Cell<Vector3D>,

	dir:     Cell<Vector3D>,
	speed: 	 Cell<Vector3D>,
}

impl CViewer {
	pub fn new(pos: Vector3D) -> CViewer {
		CViewer{ pos:   Cell::new(pos),
				 dir:   Cell::new(Vector3D::new(0.0, 0.0, 0.0)),
				 speed: Cell::new(Vector3D::new(0.0, 0.0, 0.0)), }
	}

	pub fn update(&self, time: f32) {
		let mut speed = self.speed.get();
		let mut pos = self.pos.get() + speed * time;

		self.speed.set(speed);
		self.pos.set(pos);
	}
}

impl CanBeCamera for CViewer {
	fn KeyboardEvent(&self, event: &glium::glutin::Event, trg: Vector3D, up: Vector3D) {
		let mut speed = self.speed.get();
		let mut dir = self.dir.get();

		match *event {
			glutin::Event::KeyboardInput(glutin::ElementState::Pressed,  _, Some(glutin::VirtualKeyCode::D)) => {
				dir.z = -3.0;
            },
            glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::D)) => {
               	dir.z =  0.0;
            },
            glutin::Event::KeyboardInput(glutin::ElementState::Pressed,  _, Some(glutin::VirtualKeyCode::A)) => {
               	dir.z =  3.0;
            },
            glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::A)) => {
               	dir.z =  0.0;
            },
            glutin::Event::KeyboardInput(glutin::ElementState::Pressed,  _, Some(glutin::VirtualKeyCode::W)) => {
               	dir.x = -3.0;
            },
            glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::W)) => {
               	dir.x =  0.0;
            },
            glutin::Event::KeyboardInput(glutin::ElementState::Pressed,  _, Some(glutin::VirtualKeyCode::S)) => {
                dir.x =  3.0;
            },
            glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::S)) => {
               	dir.x =  0.0;
            },
            _ => ()
		}
		self.speed.set(dir.x * trg + dir.z * trg.cross(up) + dir.y * Vector3D::new(0.0, 1.0, 0.0));

		self.dir.set(dir);
	}

	fn GetPosition(&self) -> Vector3D {
		self.pos.get()
	}

	fn GetSize(&self) -> Vector3D {
		Vector3D::new(0.0, 0.0, 0.0)
	}
}
