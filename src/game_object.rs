extern crate glium;

use models;
use texture::CTexture;
use program::CProgram;
use camera::CCamera;
use light::CLight;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface};
use glium::framebuffer::MultiOutputFrameBuffer;
use std::rc::Rc;
use camera::CanBeCamera;
use glium::glutin;

use math::Vector3D;
use math::Matrix4D;
use physical_object::CPhysicalObject;
use render_object::CRenderObject;
use geometry::inters;
use std::cell::Cell;

pub struct CGameObject {
	pub physical_object: CPhysicalObject,
	pub render_object:   CRenderObject,

	dir: Cell<Vector3D>,
}

impl CGameObject {
	pub fn new(display: &GlutinFacade, mdl: models::CModel, texture: &Rc<CTexture>, program: &Rc<CProgram>) -> CGameObject {
		let render_object = CRenderObject::new(display, &mdl, texture, program);
		let physical_object = CPhysicalObject::new(&mdl);

   	 	CGameObject{ physical_object: physical_object,
   	 				 render_object:   render_object,
   	 				 dir:   		  Cell::new(Vector3D::new(0.0, 0.0, 0.0)), }
	}  

	pub fn set_scale(&self, scale: Vector3D) {
		let old = self.physical_object.scale.get();
		self.physical_object.scale.set(scale);
	}

	pub fn set_pos(&self, pos: Vector3D) {
		let old = self.physical_object.position.get();
		self.physical_object.position.set(pos);
	}

	pub fn set_rotate(&self, rotate: Vector3D) {
		let old = self.physical_object.rotate.get();
		self.physical_object.rotate.set(rotate);
	}

	pub fn scale(&self, scale: Vector3D) {
		let old = self.physical_object.scale.get();
		self.physical_object.scale.set(Vector3D::new(old.x * scale.x, old.y * scale.y, old.z * scale.z));
	}

	pub fn pos(&self, pos: Vector3D) {
		let old = self.physical_object.position.get();
		self.physical_object.position.set(old + pos);
	}

	pub fn rotate(&self, rotate: Vector3D) {
		let old = self.physical_object.rotate.get();
		self.physical_object.rotate.set(old + rotate);
	}

	pub fn draw(&self, mut target: &mut MultiOutputFrameBuffer, PM: &Matrix4D, VM: &Matrix4D) {
		self.render_object.draw(target, PM, VM, &self.physical_object.getMT());
	}

	pub fn rollback(&self) {
		self.physical_object.rollback();
	}

	pub fn update(&self, time: f32) {
		self.physical_object.update(time);
	}
}

impl inters<CGameObject> for CGameObject {
	fn intersect_with(&self, other: &CGameObject) -> bool {
		self.physical_object.intersect_with(&other.physical_object)
	}
}

impl CanBeCamera for CGameObject {
	fn KeyboardEvent(&self, event: &glium::glutin::Event, trg: Vector3D, up: Vector3D) {
		let mut speed = self.physical_object.speed.get();
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
            // glutin::Event::KeyboardInput(glutin::ElementState::Pressed,  _, Some(glutin::VirtualKeyCode::Space)) => {
            //     speed.y += 5.0;
            // },
            _ => ()
		}
		self.physical_object.speed.set(dir.x * trg.projectionXOZ() + 
									   dir.z * trg.cross(up) + 
									   dir.y * Vector3D::new(0.0, 1.0, 0.0));

		self.dir.set(dir);
	}

	fn GetPosition(&self) -> Vector3D {
		self.physical_object.position.get()
	}

	fn GetSize(&self) -> Vector3D {
		self.physical_object.size
	}
}
