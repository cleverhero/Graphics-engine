extern crate glium;
extern crate cgmath;

use math::{ Vector3D, Matrix4D};
use std::time::{Duration, SystemTime};
use geometry;
use std::f64::consts;

use geometry::inters;
use camera::CCamera;
use light::CLight;
use light::CDirectionLight;
use shell::CShell;
use pool::Pool;
use game_object::CGameObject;
use render_object::Render;
use models::CModel;
use math::Vertex;
use math::VertexPT;
use std::rc::Rc;
use glium::index::PrimitiveType;
use glium::texture::UncompressedFloatFormat::F32F32F32F32;
use glium::texture::DepthFormat;
use glium::texture::MipmapsOption::NoMipmap;
use glium::framebuffer::MultiOutputFrameBuffer;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use viewer::CViewer;
use texture::CTexture;
use program::CProgram;
use camera::CanBeCamera;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use glutin::ElementState::Pressed;
use glutin::Event::{KeyboardInput, MouseInput};

pub struct CWorld {
	PerspectiveMatrix: Matrix4D,
	Camera:            CCamera,
	Viewer:            Rc<CViewer>,

	pub objs:          Vec<Rc<CGameObject>>,
	pub lights:        Vec<CLight>,
	pub dirlights:     Vec<CDirectionLight>,

	prog: 			   Rc<CProgram>,
	prog2: 			   Rc<CProgram>,
	lightprog:         Rc<CProgram>,
	dirlightprog:      Rc<CProgram>,

	wallTexture:       Rc<CTexture>,
	blockTexture:      Rc<CTexture>,

	OR:                [[f32; 4]; 4],

	timer:             SystemTime,
}

impl CWorld {
	pub fn new(display: &GlutinFacade, winWidth: u32, winHeight: u32) -> CWorld {
		implement_vertex!(Vertex, position, tex_coord, normal);
		implement_vertex!(VertexPT, position, tex_coord);

		let mut PerspectiveMatrix = Matrix4D::PerspectiveMatrix(60.0f32, winWidth as f32, winHeight as f32, 0.01, 100.0);
		let mut Camera = CCamera::new( Vector3D::new(0.0, -0.3, 3.0),
									   Vector3D::new(0.0,  0.0, 1.0),
									   Vector3D::new(0.0,  1.0, 0.0) );

		let texture = Rc::new( CTexture::load(display, "images/Wall.jpg") );
		let block = Rc::new( CTexture::load(display, "images/Block.jpg") );

		let prog = Rc::new( CProgram::load(display, "Shaders/GBufferV.vs", "Shaders/GBufferF.fs") );
		let prog2 = Rc::new( CProgram::load(display, "Shaders/CompositionV.vs", "Shaders/CompositionF.fs") );
		let lightprog = Rc::new( CProgram::load(display, "Shaders/LightV.vs", "Shaders/LightF.fs") );
		let dirlightprog = Rc::new( CProgram::load(display, "Shaders/LightV.vs", "Shaders/DirLightF.fs") );

		let mut light = CLight::new();
		light.range = (consts::PI/6.0) as f32;

		let mut light2 = CLight::new();
		light.range = (consts::PI/6.0) as f32;

		let mut dirlight = CDirectionLight::new();

		let timer = SystemTime::now();

		let ortho_matrix: cgmath::Matrix4<f32> = cgmath::ortho(0.0, 800.0, 0.0, 600.0, -1.0, 1.0);
    	let OR = Into::<[[f32; 4]; 4]>::into(ortho_matrix);

		let cube1 = Rc::new( CGameObject::new(display, CModel::cube(Vector3D::new(1.0, 1.0, 1.0)), &block, &prog) );
		cube1.set_scale(Vector3D::new(3.0, 3.0, 3.0));
		cube1.set_pos(Vector3D::new(-4.0, 1.0, 0.0));
		

    	let cube2 = Rc::new( CGameObject::new(display, CModel::cube(Vector3D::new(1.0, 1.0, 1.0)), &block, &prog) );
    	cube2.set_scale(Vector3D::new(0.3, 0.3, 0.3));
    	cube2.set_pos(Vector3D::new(-2.0, -0.35, -5.0));

    	let mut Viewer = Rc::new( CViewer::new( Vector3D::new(0.0, -0.3, 3.0)));

    	Camera.SetOwner(Box::new(Viewer.clone()));

    	let floor = Rc::new( CGameObject::new(display, CModel::cube(Vector3D::new(500.0, 1.0, 500.0)), &texture, &prog) );
    	floor.set_pos(Vector3D::new(0.0, -1.0, 0.0));

    	CWorld { PerspectiveMatrix: PerspectiveMatrix,
    			 Camera:            Camera,
    			 Viewer:            Viewer.clone(),

    			 objs:              vec![ cube1.clone(),
				 					      cube2.clone(), 
				 					      floor.clone(), ], 

				 lights:            vec![ light,
				 						  light2, ],

				 dirlights:         vec![ dirlight ],


    			 wallTexture:       texture.clone(),
    			 blockTexture:      block.clone(),
    			 prog:              prog.clone(),
    			 prog2:             prog2.clone(),
    			 lightprog:         lightprog.clone(),
    			 dirlightprog:      dirlightprog.clone(),
				 OR:                OR,

    			 timer:             timer, }
	}

	fn create_gbuffer(&self, mut gbuffer: &mut MultiOutputFrameBuffer) {
		let pos = -self.Camera.GetPos();
		let CameraTranslationTrans = Matrix4D::Translation(-self.Camera.GetPos());
		let CameraRotateTrans = Matrix4D::InitCameraTransform(self.Camera.target, self.Camera.up);
		let CameraTrans = CameraRotateTrans * CameraTranslationTrans;

	    for obj in &self.objs {
	    	obj.draw(&mut gbuffer, &self.PerspectiveMatrix, &CameraTrans);
	    }
	}

	fn create_lightbuffer(&self, display: &GlutinFacade, mut lightbuffer: &mut SimpleFrameBuffer, pos_texture: &Texture2d, norm_texture: &Texture2d) {
		let Verteces = [
			VertexPT{ position: [0.0,   0.0,   0.0], tex_coord: [0.0, 0.0] },
        	VertexPT{ position: [800.0, 0.0,   0.0], tex_coord: [1.0, 0.0] },
        	VertexPT{ position: [800.0, 600.0, 0.0], tex_coord: [1.0, 1.0] },
        	VertexPT{ position: [0.0,   600.0, 0.0], tex_coord: [0.0, 1.0] },
		];

		let quad_vertex_buffer = glium::VertexBuffer::new(display, &Verteces).unwrap();
    	let quad_index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[1, 0, 2, 0, 2, 3u16]
    		).unwrap();

    	let draw_params = glium::DrawParameters {
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One
                },
                constant_value: (1.0, 1.0, 1.0, 1.0)
            },
            .. Default::default()
		};

	    for light in &self.lights {
	    	let uniforms = uniform! {
	    		matrix:            self.OR,
    			light_pos:         light.pos.as_arr(),
    			light_color:       light.color.as_arr(),
    			light_attenuation: light.attenuation.as_arr(),
    			light_vector:      light.vector.as_arr(),
    			light_range:       light.range,
    			light_maxradius:   light.maxradius,
    			pos_texture:       pos_texture,
    			norm_texture:      norm_texture,
			};

			lightbuffer.draw(&quad_vertex_buffer, &quad_index_buffer, self.lightprog.prog_object(), &uniforms, &draw_params).unwrap();
		}

		for light in &self.dirlights {
	    	let uniforms = uniform! {
	    		matrix:            self.OR,
    			light_color:       light.color.as_arr(),
    			light_vector:      light.vector.as_arr(),
    			pos_texture:       pos_texture,
    			norm_texture:      norm_texture,
			};

			lightbuffer.draw(&quad_vertex_buffer, &quad_index_buffer, self.dirlightprog.prog_object(), &uniforms, &draw_params).unwrap();
		}
	}

	fn combine_buffers(&self, display: &GlutinFacade, canvas: &mut glium::Frame, decal_texture: &Texture2d, light_texture: &Texture2d) {
		let Verteces = [
			VertexPT{ position: [0.0,   0.0,   0.0], tex_coord: [0.0, 0.0] },
        	VertexPT{ position: [800.0, 0.0,   0.0], tex_coord: [1.0, 0.0] },
        	VertexPT{ position: [800.0, 600.0, 0.0], tex_coord: [1.0, 1.0] },
        	VertexPT{ position: [0.0,   600.0, 0.0], tex_coord: [0.0, 1.0] },
		];

		let quad_vertex_buffer = glium::VertexBuffer::new(display, &Verteces).unwrap();
    	let quad_index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[1, 0, 2, 0, 2, 3u16]
    		).unwrap();

	    let uniforms = uniform! {
            matrix:           self.OR,
            decal_texture:    decal_texture,
            lighting_texture: light_texture
        };

		canvas.draw(&quad_vertex_buffer, &quad_index_buffer, self.prog2.prog_object(), &uniforms, &Default::default()).unwrap();
	}

	pub fn draw(&self, display: &GlutinFacade, mut render: &mut Render) {
		let mut canvas = display.draw();

		render.gbuffer.clear_color_and_depth((0.0, 0.7, 0.933, 0.0), 1.0);
		render.light_buffer.clear_color_and_depth((0.005, 0.005, 0.005, 0.0), 1.0);
		canvas.clear_color(0.0, 0.0, 0.0, 0.0);
		
    	self.create_gbuffer(&mut render.gbuffer);
    	self.create_lightbuffer(display, &mut render.light_buffer, &render.pos_texture, &render.norm_texture);
		self.combine_buffers(display, &mut canvas, &render.text_texture, &render.light_texture);
		canvas.finish().unwrap();
	}

	fn create_new_obj(&mut self, display: &GlutinFacade) {
		self.objs.push(Rc::new( CGameObject::new(display, CModel::cube(Vector3D::new(1.0, 1.0, 1.0)), &self.blockTexture, &self.prog) ));

        let top = self.objs.len() - 1;
        let mut new_pos = self.Camera.GetPos() - self.Camera.target.projectionXOZ() * 5.0;
        new_pos.y = -0.35;

        let mut angle = Vector3D::new(0.0, 0.0, 0.0);
        let trg = self.Camera.target.normalize();

        self.objs[top].set_pos(new_pos); 
        self.objs[top].set_rotate(angle); 
        self.objs[top].set_scale(Vector3D::new(0.3, 0.3, 0.3));
	}

	fn create_new_lightsource(&mut self, display: &GlutinFacade) {
		self.lights.push(CLight::new());

        let top = self.lights.len() - 1;
        self.lights[top].set_pos(self.Camera.GetPos()); 
	}

	pub fn checkEvents(&mut self, event: &glium::glutin::Event, display: &GlutinFacade) {
		self.Camera.onKeyboard(event);
		self.Camera.onMouseMove(event, display);

		match *event {
			KeyboardInput(Pressed,  _, Some(glutin::VirtualKeyCode::C)) => {
                self.create_new_obj(display);
            },
            KeyboardInput(Pressed,  _, Some(glutin::VirtualKeyCode::M)) => {
                self.Camera.SetOwner(Box::new(self.objs[1].clone()));
            },
            KeyboardInput(Pressed,  _, Some(glutin::VirtualKeyCode::V)) => {
                self.Camera.SetOwner(Box::new(self.Viewer.clone()));
            },
            MouseInput(Pressed, glutin::MouseButton::Left) => {
				self.create_new_obj(display);
            },
            MouseInput(Pressed, glutin::MouseButton::Right) => {
				self.create_new_lightsource(display);
            },
            _ => ()
        }
	}

	pub fn update(&mut self) {
		let new_timer = SystemTime::now();
		let diff = new_timer.duration_since(self.timer).unwrap();
		let t = (diff.subsec_nanos() as f32) / 1000000000.0;

		self.Camera.update(t);
		self.lights[0].pos = self.Camera.GetPos();
		self.lights[0].vector = self.Camera.target;
		self.Viewer.update(t);

		println!("{}", t);

		for i in (0..self.objs.len()) {
			let obj_i = &self.objs[i];
	    	obj_i.update(t);

	    	for j in (0..self.objs.len()) {
	    		if i == j { continue; }
	    		let obj_j = &self.objs[j];
	    		if obj_i.intersect_with(obj_j) {
	    			obj_i.rollback();
	    			break;
	    		}
	    	}
	    }

		self.timer = new_timer;
	}
}