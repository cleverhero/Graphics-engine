use math::Vector3D;
use math::Matrix4D;

pub trait inters<T> {
	fn intersect_with(&self, other: &T) -> bool;
}

pub struct Line {
	pub verteces: [Vector3D; 2]
}

pub struct Triangle {
	pub verteces: [Vector3D; 3]
}

pub struct AABB {
	pub minV: Vector3D,
	pub maxV: Vector3D
}

impl AABB {
	pub fn new(coords: &Vec<Vector3D>, MWorld: Matrix4D) -> AABB {
		let mut max = Vector3D::new(-100000.0, -100000.0, -100000.0);
		let mut min = Vector3D::new( 100000.0,  100000.0,  100000.0);

		for v in coords {
			if v.x < min.x { min.x = v.x; }
			if v.y < min.y { min.y = v.y; }
			if v.z < min.z { min.z = v.z; }

			if v.x > max.x { max.x = v.x; }
			if v.y > max.y { max.y = v.y; }
			if v.z > max.z { max.z = v.z; }
		}

		return AABB{ minV: min * MWorld,
					 maxV: max * MWorld }
	}
}

impl inters<Line> for Triangle {
	fn intersect_with(&self, other: &Line) -> bool {
		let mut tp0 = Vector3D::new(0.0, 0.0, 0.0);
		let mut tp1 = self.verteces[1] - self.verteces[0];
		let mut tp2 = self.verteces[2] - self.verteces[0];

		let mut lp1 = other.verteces[0] - self.verteces[0];
		let mut lp2 = other.verteces[1] - self.verteces[0];

		let A =  tp1.y*tp2.z - tp2.y*tp1.z;
		let B = -tp1.x*tp2.z + tp2.x*tp1.z;
		let C =  tp1.x*tp2.y - tp2.x*tp1.y;

		let mut n = Vector3D::new(A, B, C).normalize();
		let angle = n.angle(&Vector3D::new(0.0, 1.0, 0.0)).to_degrees();
		let right = n.cross(&Vector3D::new(0.0, 1.0, 0.0)).normalize();

		tp0.rotate(angle, &right); 
		tp1.rotate(angle, &right);
		tp2.rotate(angle, &right);
		lp1.rotate(angle, &right);
		lp2.rotate(angle, &right);

		if (lp1.y * lp2.y > 0.0) {return false};
		let k = (lp1.y - lp2.y)/(lp1.y);
		let np = lp1 - (lp1 - lp2)*k;

		let c1 = (tp0.x - np.x) * (tp0.z - tp0.z) - (tp0.x - tp0.x) * (tp0.z - np.z);
		let c2 = (tp1.x - np.x) * (tp1.z - tp1.z) - (tp1.x - tp1.x) * (tp1.z - np.z);
		let c3 = (tp2.x - np.x) * (tp2.z - tp2.z) - (tp2.x - tp2.x) * (tp2.z - np.z);

		if ((c1 > 0.0 && c2 > 0.0 && c3 > 0.0) || (c1 < 0.0 && c2 < 0.0 && c3 < 0.0) || (c1 * c2 * c3 == 0.0)) { return true };

		false
	}
}

impl inters<Triangle> for Triangle {
	fn intersect_with(&self, other: &Triangle) -> bool {
		for (i, j) in vec!((0, 1), (1, 2), (0, 2)) {
			let line = Line{ verteces: [other.verteces[i], other.verteces[j]] };
			if (self.intersect_with(&line)) { return true };

			let line = Line{ verteces: [self.verteces[i], self.verteces[j]] };
			if (other.intersect_with(&line)) { return true };
		} 

		false
	}
}

impl inters<Triangle> for Line {
	fn intersect_with(&self, other: &Triangle) -> bool {
		other.intersect_with(self)
	}
}

impl inters<AABB> for AABB {
	fn intersect_with(&self, other: &AABB) -> bool {
		return (self.minV.x < other.maxV.x && self.maxV.x > other.minV.x) && 
			   (self.minV.y < other.maxV.y && self.maxV.y > other.minV.y) &&
               (self.minV.z < other.maxV.z && self.maxV.z > other.minV.z)
    }
}
