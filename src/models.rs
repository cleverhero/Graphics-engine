extern crate glium;

use math::Vertex;
use math::Vector3D;
use glium::backend::glutin_backend::GlutinFacade;

pub struct CModel {
	pub vertcs: Vec<Vertex>,
    pub coords: Vec<Vector3D>,
    pub inds:   Vec<u32>,
    pub name:   String,
} 

impl CModel {
	pub fn new(vertcs: &Vec<Vertex>, inds: &Vec<u32>, coords: &Vec<Vector3D>, name: String) -> CModel {
		CModel{ vertcs: vertcs.clone(), 
                coords: coords.clone(),
                inds:   inds.clone(),
                name:   name  }
	}

	pub fn cube(size: Vector3D) -> CModel {
        let s = size * (1.0/2.0);

        let coords = vec![ Vector3D::new( -s.x, -s.y, -s.z ),
                           Vector3D::new( -s.x,  s.y, -s.z ),
                           Vector3D::new(  s.x, -s.y, -s.z ),
                           Vector3D::new(  s.x,  s.y, -s.z ),
                           Vector3D::new( -s.x, -s.y,  s.z ),
                           Vector3D::new( -s.x,  s.y,  s.z ),
                           Vector3D::new(  s.x, -s.y,  s.z ),
                           Vector3D::new(  s.x,  s.y,  s.z ), ];
                  

		let vertcs = vec![ Vertex::new( [ -s.x, -s.y, -s.z ], [ 0.0,    0.0    ], [  0.0,  0.0, -1.0 ] ),
    			           Vertex::new( [ -s.x,  s.y, -s.z ], [ 0.0,    size.y ], [  0.0,  0.0, -1.0 ] ),
    			           Vertex::new( [  s.x, -s.y, -s.z ], [ size.x, 0.0    ], [  0.0,  0.0, -1.0 ] ),
    			           Vertex::new( [  s.x,  s.y, -s.z ], [ size.x, size.y ], [  0.0,  0.0, -1.0 ] ),
   
    			           Vertex::new( [ -s.x, -s.y,  s.z ], [ 0.0,    0.0    ], [  0.0,  0.0,  1.0 ] ),
    			           Vertex::new( [ -s.x,  s.y,  s.z ], [ 0.0,    size.y ], [  0.0,  0.0,  1.0 ] ),
    			           Vertex::new( [  s.x, -s.y,  s.z ], [ size.x, 0.0    ], [  0.0,  0.0,  1.0 ] ),
    			           Vertex::new( [  s.x,  s.y,  s.z ], [ size.x, size.y ], [  0.0,  0.0,  1.0 ] ),
 
    			           Vertex::new( [ -s.x, -s.y, -s.z ], [ 0.0,    0.0    ], [ -1.0,  0.0,  0.0 ] ),
    			           Vertex::new( [ -s.x,  s.y, -s.z ], [ 0.0,    size.y ], [ -1.0,  0.0,  0.0 ] ),
    			           Vertex::new( [ -s.x, -s.y,  s.z ], [ size.z, 0.0    ], [ -1.0,  0.0,  0.0 ] ),
    			           Vertex::new( [ -s.x,  s.y,  s.z ], [ size.z, size.y ], [ -1.0,  0.0,  0.0 ] ),
    
    			           Vertex::new( [  s.x, -s.y, -s.z ], [ 0.0,    0.0    ], [  1.0,  0.0,  0.0 ] ),
    			           Vertex::new( [  s.x,  s.y, -s.z ], [ 0.0,    size.y ], [  1.0,  0.0,  0.0 ] ),
    			           Vertex::new( [  s.x, -s.y,  s.z ], [ size.z, 0.0    ], [  1.0,  0.0,  0.0 ] ),
    			           Vertex::new( [  s.x,  s.y,  s.z ], [ size.z, size.y ], [  1.0,  0.0,  0.0 ] ),
 
    			           Vertex::new( [ -s.x, -s.y, -s.z ], [ 0.0,    0.0    ], [  0.0, -1.0,  0.0 ] ),
    			           Vertex::new( [ -s.x, -s.y,  s.z ], [ 0.0,    size.z ], [  0.0, -1.0,  0.0 ] ),
    			           Vertex::new( [  s.x, -s.y, -s.z ], [ size.x, 0.0    ], [  0.0, -1.0,  0.0 ] ),
    			           Vertex::new( [  s.x, -s.y,  s.z ], [ size.x, size.z ], [  0.0, -1.0,  0.0 ] ),
    
    			           Vertex::new( [ -s.x,  s.y, -s.z ], [ 0.0,    0.0    ], [  0.0,  1.0,  0.0 ] ),
    			           Vertex::new( [ -s.x,  s.y,  s.z ], [ 0.0,    size.z ], [  0.0,  1.0,  0.0 ] ),
    			           Vertex::new( [  s.x,  s.y, -s.z ], [ size.x, 0.0    ], [  0.0,  1.0,  0.0 ] ),
    			           Vertex::new( [  s.x,  s.y,  s.z ], [ size.x, size.z ], [  0.0,  1.0,  0.0 ] ), ];

    	let inds = vec![  1,  0,  2,
    					  1,  2,  3,
 
    					  4,  5,  6,
    					  6,  5,  7,
 
    					  8,  9, 10,
    					 10,  9, 11,
 
    					 13, 12, 14,
    					 13, 14, 15,
 
    					 16, 17, 18,
    					 18, 17, 19,
 
    					 21, 20, 22,
    					 21, 22, 23 ];

    	CModel::new(&vertcs, &inds, &coords, "c".into())
	}
}