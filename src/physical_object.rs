use math::Vector3D;
use math::Matrix4D;

use models;
use geometry::inters;
use geometry::AABB;
use glium::glutin;
use std::cell::Cell;

pub struct CPhysicalObject {
	pub position:     Cell<Vector3D>,
	pub old_position: Cell<Vector3D>,
	pub scale:        Cell<Vector3D>,
	pub rotate:       Cell<Vector3D>,

	pub speed:        Cell<Vector3D>,
	pub power:        Cell<Vector3D>,

	pub coords:       Vec<Vector3D>,
	pub inds:         Vec<u32>,

	pub size:         Vector3D,
	pub weight:       f32,
	pub movable:      bool,
}

impl CPhysicalObject {
	pub fn new(mdl: &models::CModel, weight: f32) -> CPhysicalObject {
		let coords = mdl.coords.clone();
		let sb = AABB::new(&coords, Matrix4D::InitIdentity());


   	 	CPhysicalObject{ position:     Cell::new(Vector3D::new(0.0, 0.0, 0.0)), 
   	 		 			 old_position: Cell::new(Vector3D::new(0.0, 0.0, 0.0)), 
   	 		     	     scale:        Cell::new(Vector3D::new(1.0, 1.0, 1.0)),
   	 		             rotate:       Cell::new(Vector3D::new(0.0, 0.0, 0.0)), 
   	 		             speed:        Cell::new(Vector3D::new(0.0, 0.0, 0.0)), 
   	 		             power:        Cell::new(Vector3D::new(0.0, 0.0, 0.0)), 


   	 		         	 coords:       coords,
						 inds:         mdl.inds.clone(),
						 size:         sb.maxV - sb.minV,
						 weight:       weight,
						 movable:      false }
	}  

	pub fn getMT(&self) -> Matrix4D {
		Matrix4D::Translation(&self.position.get()) * Matrix4D::Scale(&self.scale.get()) * Matrix4D::Rotate(&self.rotate.get())
	}

	pub fn add_power(&self, power: &Vector3D) {
		if (!self.movable) { return }
		self.power.set(self.power.get() + *power);
	}

	pub fn add_speed(&self, speed: &Vector3D) {
		if (!self.movable) { return }
		self.speed.set(self.speed.get() + *speed);
	}

	pub fn init(&self) {
		self.power.set(Vector3D::new(0.0, 0.0, 0.0));

		//self.add_power(&Vector3D::new(0.0, -9.8*self.weight, 0.0));
	}

	pub fn rollback(&self) {
		let pos = self.old_position.get();
		self.position.set(pos);
	}

	pub fn update(&self, time: f32) {
		let mut speed = self.speed.get();
		let a = self.power.get()*(1.0/self.weight);


		let mut pos = self.position.get();
		self.old_position.set(pos);
		pos = self.position.get() + speed * time + a*(time*time/2.0);
		self.add_speed(&(a*time));

		self.position.set(pos);
	}

	pub fn to_string(&self) -> String {
		self.position.get().to_string() + &" " + &self.scale.get().to_string() + &" " + &self.speed.get().to_string()
	}

	pub fn collision(&self, other: &CPhysicalObject) {

	}

	// pub fn go_x_z(&mut self, time: f32) {
	// 	self.old_position = self.position;
	// 	self.position += (self.target.projectionXOZ() * self.speed.z + self.target.cross(self.up) * self.speed.x) * time;
	// }

	// pub fn rollback_go_x_z(&mut self) {
	// 	self.position = self.old_position;
	// }

	// pub fn go_y(&mut self, time: f32) {
	// 	self.old_position.y = self.position.y;
	// 	self.position.y += self.speed.y * time  - (9.8 * time*time / 2.0);
	// }

	// pub fn rollback_go_y(&mut self) {
	// 	self.position.y = self.old_position.y;
	// 	self.speed.y = 0.0;
	// }
}

impl inters<CPhysicalObject> for CPhysicalObject {
	fn intersect_with(&self, other: &CPhysicalObject) -> bool {
		let shell1 = AABB::new(&self.coords, self.getMT());
		let shell2 = AABB::new(&other.coords, other.getMT());

		shell1.intersect_with(&shell2)
	}
}