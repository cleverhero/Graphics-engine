extern crate glium;

use glium::glutin;
use math::{ Vector3D, Matrix4D};
use std::rc::Rc;
use math::Point;
use std::f64;
use std::cmp::{min, max};
use std::cell::RefCell;

use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface};


pub trait CanBeCamera {
	fn KeyboardEvent(&self, event: &glium::glutin::Event, trg: Vector3D, up: Vector3D);
	fn GetPosition(&self) -> Vector3D;
	fn GetSize(&self) -> Vector3D;
}

pub struct CCamera {
	pub PerspectiveMatrix: Matrix4D,

	pub target:   	Vector3D,
	pub up:       	Vector3D,
	
	player:			Box<Rc<CanBeCamera>>,
	
	angle:        	Point,
	mousePos:     	Point,

	shift: 			Vector3D,
	dist:           f32,
}	

struct DefaultCamera {
	pub pos: Vector3D,
}  

impl DefaultCamera {
	fn new() -> DefaultCamera {
		DefaultCamera{ pos:   Vector3D::new(0.0, 0.0, 0.0) }
	}

	fn update(&mut self, time: f32) {}
}

impl CanBeCamera for DefaultCamera {
	fn KeyboardEvent(&self, event: &glium::glutin::Event, trg: Vector3D, up: Vector3D) {}
	fn GetPosition(&self) -> Vector3D {	self.pos }
	fn GetSize(&self) -> Vector3D {	Vector3D::new(0.0, 0.0, 0.0) }
}

impl CCamera {
	pub fn new(pos: Vector3D, trg: Vector3D, up: Vector3D, w: u32, h: u32) -> CCamera {
		let mut PerspectiveMatrix = Matrix4D::PerspectiveMatrix(60.0f32, w as f32, h as f32, 0.01, 100.0);

		CCamera{ target:       trg,
			     up:           up,
			     player:       Box::new(Rc::new( DefaultCamera::new() )),
			     angle:        Point::new(0.0, 0.0),
			     mousePos:     Point::new(w as f32 / 2.0, h as f32 / 2.0), 
			     shift:        Vector3D::new(0.0, 0.0, 0.0),
			     dist:         2.0,

			     PerspectiveMatrix: PerspectiveMatrix }
	}

	pub fn SetOwner(&mut self, player: Box<Rc<CanBeCamera>>) {
		self.shift = player.GetSize() * 0.5;
		self.player = player;
	}

	pub fn onKeyboard(&mut self, event: &glium::glutin::Event) {
		self.player.KeyboardEvent(event, self.target, self.up);
	}

	pub fn GetPos(&self) -> Vector3D {
		self.player.GetPosition() + self.target * self.dist * self.shift.y
	}

	pub fn onMouseMove(&mut self, event: &glium::glutin::Event, display: &GlutinFacade) {
		match *event {
			glutin::Event::MouseMoved(x, y) => {
				let newMousePos = Point::new(x as f32, y as f32);

				let mut delta = self.mousePos - newMousePos;

				if (delta.x.abs() < 1.2 ) { delta.x = 0.0; }
				if (delta.y.abs() < 1.2 ) { delta.y = 0.0; }

				self.angle += delta / 20.0;
				if self.angle.y >  90.0 { self.angle.y =  90.0; }
				if self.angle.y < -90.0 { self.angle.y = -90.0; }
			}
			glium::glutin::Event::MouseWheel(delta, display) => match delta {
    			glium::glutin::MouseScrollDelta::LineDelta(x, y)  => {
    				self.dist += y / 10.0;
    			},
    			glium::glutin::MouseScrollDelta::PixelDelta(x, y) => {},
			},
			_ => return
		}
		
	}

	pub fn update(&mut self, time: f32) {
		let Vaxis = Vector3D::new(0.0, 1.0, 0.0);

    	let mut View = Vector3D::new(0.0, 0.0, 1.0);
    	View.rotate(self.angle.x, &Vaxis);
    	View.normalize();
	
    	let Haxis = Vaxis.cross(&View);
    	Haxis.normalize();
    	View.rotate(self.angle.y, &Haxis);

    	self.target = View;
    	self.target.normalize();
	
    	self.up = View.cross(&Haxis);
		self.up.normalize();
	}
}