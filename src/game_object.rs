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
use models::CModel;

use math::Vector3D;
use math::Matrix4D;
use physical_object::CPhysicalObject;
use render_object::CRenderObject;
use geometry::inters;
use std::cell::Cell;

use std::f32;
use std::str::FromStr;

pub struct CGameObject {
	pub physical_object: CPhysicalObject,
	pub render_object:   CRenderObject,

	dir: Cell<Vector3D>,
}

impl CGameObject {
	pub fn new(display: &GlutinFacade, mdl: models::CModel, texture: &Rc<CTexture>, program: &Rc<CProgram>) -> CGameObject {
		let render_object = CRenderObject::new(display, &mdl, texture, program);
		let physical_object = CPhysicalObject::new(&mdl, 1.0);

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

	pub fn init(&self) {
		self.physical_object.init();
	}

	pub fn collision(&self, other: &CGameObject) {
		self.physical_object.collision(&other.physical_object);
	}

	pub fn save(&self) -> String {
		self.render_object.to_string() + &" " + &self.physical_object.to_string()
	}

	pub fn load(display: &GlutinFacade, data: String, textures: &Vec<Rc<CTexture>>, prog: &Rc<CProgram>) -> CGameObject {
		let items: Vec<&str> = data.split(" ").collect();
		let model_name: &str = items[0];
		let tex_id = f32::from_str(items[1]).unwrap() as i32;
		let mut cur_tex = textures[0].clone();
		for texture in textures {
			if (tex_id == texture.id) { 
				cur_tex = texture.clone();
				break;
			}
		}
		let pos = Vector3D::new(
			f32::from_str(items[2]).unwrap(),
			f32::from_str(items[3]).unwrap(),
			f32::from_str(items[4]).unwrap()
		);
		let scale = Vector3D::new(
			f32::from_str(items[5]).unwrap(),
			f32::from_str(items[6]).unwrap(),
			f32::from_str(items[7]).unwrap()
		);
		let speed = Vector3D::new(
			f32::from_str(items[8]).unwrap(),
			f32::from_str(items[9]).unwrap(),
			f32::from_str(items[10]).unwrap()
		);
		let mut new_obj: CGameObject;
		if (model_name == "c") {
			new_obj = CGameObject::new(display, CModel::cube(Vector3D::new(1.0, 1.0, 1.0)), &cur_tex, prog);
		} else {
			new_obj = CGameObject::new(display, CModel::cube(Vector3D::new(1.0, 1.0, 1.0)), &cur_tex, prog);
		}
		new_obj.set_scale(scale);
		new_obj.set_pos(pos);
		new_obj
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
									   dir.z * trg.cross(&up) + 
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
