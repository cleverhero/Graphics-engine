use math::Vector3D;

pub struct CShell {
	min_vertex: Vector3D,
	max_vertex: Vector3D
}

fn inters(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
	if (min1 > max2) { return false; }
	if (max1 < min2) { return false; }

	true
}

impl CShell {
	pub fn new(size: Vector3D, pos: Vector3D, scale: Vector3D) -> CShell {
		CShell{ min_vertex: Vector3D::new( pos.x - size.x * scale.x, 
										   pos.y - size.y * scale.y, 
										   pos.z - size.z * scale.z ),
			    max_vertex: Vector3D::new( pos.x + size.x * scale.x, 
										   pos.y + size.y * scale.y, 
										   pos.z + size.z * scale.z ), }
	}

	pub fn inters(&self, other: &CShell) -> bool {
		if (!inters(self.min_vertex.x, self.max_vertex.x, other.min_vertex.x, other.max_vertex.x)) { return false; }
		if (!inters(self.min_vertex.y, self.max_vertex.y, other.min_vertex.y, other.max_vertex.y)) { return false; }
		if (!inters(self.min_vertex.z, self.max_vertex.z, other.min_vertex.z, other.max_vertex.z)) { return false; }

		true
	}
}