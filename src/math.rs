use std::f64;
use std::f64::consts;
use std::ops::{Mul, Div, AddAssign, Add, Sub, Neg};

use cgmath::prelude::*;
use cgmath::Rad;

#[derive(Clone, Copy, Debug)]
pub struct Size2 {
    pub w: u32,
    pub h: u32,
}

//---------------------------------------------------------------------------------------------------
//------------------------------------------------VERTEX---------------------------------------------
//---------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position:  [f32; 3],
    pub tex_coord: [f32; 2],
    pub normal:    [f32; 3],
}

#[derive(Copy, Clone)]
pub struct VertexPT {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2]
}

impl Vertex {
	pub fn new(pos: [f32; 3], tex_coord: [f32; 2], normal: [f32; 3]) -> Vertex {
		Vertex { position:  pos, 
				 tex_coord: tex_coord, 
				 normal:    normal }
	}
}

//---------------------------------------------------------------------------------------------------
//------------------------------------------------POINT----------------------------------------------
//---------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Point {
	pub x: f32,
	pub y: f32
}

impl Point {
	pub fn new(nx: f32, ny: f32) -> Point {
		Point { x: nx, y: ny }
	}
}

impl Sub for Point {
	type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Div<f32> for Point {
	type Output = Point;

    fn div(self, other: f32) -> Point {
        Point {
            x: self.x/other,
            y: self.y/other,
        }
    }
}

//---------------------------------------------------------------------------------------------------
//------------------------------------------------VECTOR3D-------------------------------------------
//---------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Vector3D {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vector3D {
	pub fn new(nx: f32, ny: f32, nz: f32) -> Vector3D {
		Vector3D { x: nx, y: ny, z: nz }
	}

	pub fn as_arr(&self) -> [f32; 3] {
		[self.x, self.y, self.z]
	}

	pub fn cross(&self, v: &Vector3D) -> Vector3D {
		let x = self.y * v.z - self.z * v.y;
    	let y = self.z * v.x - self.x * v.z;
    	let z = self.x * v.y - self.y * v.x;

    	Vector3D { x: x, y: y, z: z }
	}

	pub fn length(&self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn normalize(mut self) -> Vector3D {
		let Length = self.length();
		if Length < 0.0000001 { return Vector3D::new(0.0, 0.0, 0.0); }
 
    	self.x /= Length;
    	self.y /= Length;
    	self.z /= Length;

    	self
	}

	pub fn to_string(&self) -> String {
		self.x.to_string() + &" " + &self.y.to_string() + &" " + &self.z.to_string()
	}

	pub fn rotate(&mut self, angle: f32, axis: &Vector3D) {
		let mut quaternion = Quaternion{ a: (angle / 2.0 * (consts::PI as f32) / 180.0 ).cos(), vector: *axis * (angle / 2.0 * (consts::PI as f32) / 180.0 ).sin() };

		let newV = (quaternion * Quaternion{ a: 0.0, vector: *self } * (-quaternion)).vector_part();
		*self = newV;
	}

	pub fn projectionXOZ(&self) -> Vector3D {
		let mut newv = Vector3D::new(self.x, 0.0, self.z);
		newv = newv.normalize();

		newv
	}

	pub fn angle(&self, other: &Vector3D) -> f32 {
		if (self.length() < 0.0000001) || (other.length() < 0.0000001) { return 0.0; }
		(((*self) * (*other)) / (self.length() * other.length())).acos()
	}
}



impl Add for Vector3D {
	type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
	type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3D {
	type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Vector3D) {
        *self = Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul<f32> for Vector3D {
	type Output = Vector3D;

    fn mul(self, other: f32) -> Vector3D {
        Vector3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Matrix4D> for Vector3D {
	type Output = Vector3D;

    fn mul(self, other: Matrix4D) -> Vector3D {
        let mut ans = Vector3D::new(0.0, 0.0, 0.0);
        
        ans.x = other.matrix[0][0] * self.x + other.matrix[1][0] * self.y + other.matrix[2][0] * self.z + other.matrix[3][0];
        ans.y = other.matrix[0][1] * self.x + other.matrix[1][1] * self.y + other.matrix[2][1] * self.z + other.matrix[3][1];
        ans.z = other.matrix[0][2] * self.x + other.matrix[1][2] * self.y + other.matrix[2][2] * self.z + other.matrix[3][2];

        ans
    }
}

impl Mul<Vector3D> for f32 {
	type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul for Vector3D {
	type Output = f32;

	fn mul(self, other: Vector3D) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
}

//---------------------------------------------------------------------------------------------------
//------------------------------------------------MATRIX4D-------------------------------------------
//---------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Matrix4D {
	pub matrix: [[f32; 4]; 4],
}

impl Matrix4D {
	pub fn InitIdentity() -> Matrix4D {
		let m = [ [ 1.0, 0.0, 0.0, 0.0f32 ],
			      [ 0.0, 1.0, 0.0, 0.0f32 ],
			      [ 0.0, 0.0, 1.0, 0.0f32 ],
			      [ 0.0, 0.0, 0.0, 1.0f32 ] ];

		Matrix4D { matrix: m }
	}

	pub fn Translation(vecPos: &Vector3D) -> Matrix4D {
		let m = [ [ 1.0,      0.0,      0.0,      0.0f32 ],
			      [ 0.0,      1.0,      0.0,      0.0f32 ],
			      [ 0.0,      0.0,      1.0,      0.0f32 ],
			      [ vecPos.x, vecPos.y, vecPos.z, 1.0f32 ] ];

		Matrix4D { matrix: m }
	}

	pub fn Scale(vecScale: &Vector3D) -> Matrix4D {
		let m = [ [ vecScale.x, 0.0,        0.0,        0.0f32 ],
			      [ 0.0,        vecScale.y, 0.0,        0.0f32 ],
			      [ 0.0,        0.0,        vecScale.z, 0.0f32 ],
			      [ 0.0,        0.0,        0.0,        1.0f32 ] ];

		Matrix4D { matrix: m }
	}

	pub fn Rotate(vecRot: &Vector3D) -> Matrix4D {
		let angleX = Rad{ s: vecRot.x };
		let x = [ [ 1.0, 0.0,               0.0,              0.0f32 ],
			      [ 0.0, Rad::cos(angleX), -Rad::sin(angleX), 0.0f32 ],
			      [ 0.0, Rad::sin(angleX),  Rad::cos(angleX), 0.0f32 ],
			      [ 0.0, 0.0,               0.0,              1.0f32 ] ];

		let angleY = Rad{ s: vecRot.y };
		let y = [ [ Rad::cos(angleY), 0.0, -Rad::sin(angleY), 0.0f32 ],
			      [ 0.0 ,             1.0,  0.0,              0.0f32 ],
			      [ Rad::sin(angleY), 0.0,  Rad::cos(angleY), 0.0f32 ],
			      [ 0.0,              0.0,  0.0,              1.0f32 ] ];
			      
		let angleZ = Rad{ s: vecRot.z };
		let z = [ [ Rad::cos(angleZ), -Rad::sin(angleZ), 0.0, 0.0f32 ],
			      [ Rad::sin(angleZ),  Rad::cos(angleZ), 0.0, 0.0f32 ],
			      [ 0.0,               0.0,              1.0, 0.0f32 ],
			      [ 0.0,               0.0,              0.0, 1.0f32 ] ];    

		Matrix4D { matrix: x } * Matrix4D { matrix: y } * Matrix4D { matrix: z }
	}

	pub fn PerspectiveMatrix(a: f32, width: f32, height: f32, near: f32, far: f32) -> Matrix4D {
		let ar = width / height;
    	let Range = near - far;

    	let angle = Rad{ s: a * (consts::PI as f32) / 180.0 };
    	let tanHalfFOV = Rad::tan(angle / 2.0);


		let m = [ [ 1.0 / (ar * tanHalfFOV), 0.0,                0.0,                         0.0f32 ],
		 	      [ 0.0,                     1.0 / tanHalfFOV,   0.0,                         0.0f32 ],
		 	      [ 0.0,                     0.0,                (-near - far) / Range,       1.0f32 ], 
		 	      [ 0.0,                     0.0,                2.0f32 * far * near / Range, 0.0f32 ] ];

		Matrix4D { matrix: m }
	}

	pub fn InitCameraTransform(Target: &Vector3D, Up: &Vector3D) -> Matrix4D {
    	let mut N = -*Target;
    	N.normalize();
    	let mut U = *Up;
    	U.normalize();
    	U = U.cross(Target);
   		let mut V = Up;
 
    	Matrix4D { matrix: [ [U.x,  V.x,  N.x,  0.0f32], 
    			   		     [U.y,  V.y,  N.y,  0.0f32],
    			   		     [U.z,  V.z,  N.z,  0.0f32],
    			   		     [0.0,  0.0,  0.0,  1.0f32] ] }
	}
}

impl Mul for Matrix4D {
	type Output = Matrix4D;

	fn mul(self, other: Matrix4D) -> Matrix4D {
		let mut res = Matrix4D::InitIdentity();

		for i in (0..4) {
        	for j in (0..4) {
            	res.matrix[i][j] = self.matrix[0][j] * other.matrix[i][0] +
                      		       self.matrix[1][j] * other.matrix[i][1] +
                                   self.matrix[2][j] * other.matrix[i][2] +
                                   self.matrix[3][j] * other.matrix[i][3];
        	}
		}
  		res
	}
}

//---------------------------------------------------------------------------------------------------
//------------------------------------------------QUATERNION-----------------------------------------
//---------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
struct Quaternion {
	vector: Vector3D,
	a: f32
}


impl Quaternion {
	fn new(sclr: f32, vctr: &Vector3D) -> Quaternion {
		Quaternion { vector: *vctr, a: sclr }
	}

	fn scalar_part(self) -> f32 {
		self.a
	}

	fn vector_part(self) -> Vector3D {
		self.vector
	}
}

impl Mul for Quaternion {
	type Output = Quaternion;

	fn mul(self, other: Quaternion) -> Quaternion {
		let a = (self.a * other.a) - (self.vector.x * other.vector.x) - (self.vector.y * other.vector.y) - (self.vector.z * other.vector.z);
		let x = (self.vector.x * other.a) + (self.a * other.vector.x) + (self.vector.y * other.vector.z) - (self.vector.z * other.vector.y);
		let y = (self.vector.y * other.a) + (self.a * other.vector.y) + (self.vector.z * other.vector.x) - (self.vector.x * other.vector.z);
		let z = (self.vector.z * other.a) + (self.a * other.vector.z) + (self.vector.x * other.vector.y) - (self.vector.y * other.vector.x);

		Quaternion{ a:      a, 
					vector: Vector3D::new(x, y, z) }

	}
}

impl Neg for Quaternion {
	type Output = Quaternion;

    fn neg(self) -> Quaternion {
        Quaternion { a:       self.a, 
        			 vector: -self.vector }
    }
}