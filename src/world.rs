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
use render::Render;
use models::CModel;
use math::Vertex;
use std::rc::Rc;
use glium::index::PrimitiveType;
use glium::texture::UncompressedFloatFormat::F32F32F32F32;
use glium::texture::DepthFormat;
use glium::texture::MipmapsOption::NoMipmap;
use glium::framebuffer::MultiOutputFrameBuffer;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use viewer::CViewer;
use math::VertexPT;
use texture::CTexture;
use program::CProgram;
use camera::CanBeCamera;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use glutin::ElementState::Pressed;
use glutin::Event::{KeyboardInput, MouseInput};

pub struct ChangedProperties {
    pub backgroundLightColor: Vector3D,
    pub lightColor: Vector3D
}

impl ChangedProperties {
    pub fn new() -> ChangedProperties {
        ChangedProperties{
            backgroundLightColor: Vector3D::new(0.0005, 0.0005, 0.0005),
            lightColor: Vector3D::new(1.0, 0.0, 0.0)
        }
    }
}

pub struct CWorld {
    PerspectiveMatrix: Matrix4D,
    Camera:            CCamera,
    Viewer:            Rc<CViewer>,

    pub objs:          Vec<Rc<CGameObject>>,
    pub lights:        Vec<CLight>,
    pub dirlights:     Vec<CDirectionLight>,

    pub changedProp:   ChangedProperties,

    prog:              Rc<CProgram>,
    prog2:             Rc<CProgram>,
    lightprog:         Rc<CProgram>,
    dirlightprog:      Rc<CProgram>,

    wallTexture:       Rc<CTexture>,
    blockTexture:      Rc<CTexture>,

    timer:             SystemTime,
}

impl CWorld {
    pub fn new(display: &GlutinFacade, winWidth: u32, winHeight: u32) -> CWorld {
        implement_vertex!(Vertex, position, tex_coord, normal);
        implement_vertex!(VertexPT, position, tex_coord);

        let mut PerspectiveMatrix = Matrix4D::PerspectiveMatrix(60.0f32, winWidth as f32, winHeight as f32, 0.01, 100.0);
        let mut Camera = CCamera::new( Vector3D::new(0.0, -0.3, 3.0),
                                       Vector3D::new(0.0,  0.0, 1.0),
                                       Vector3D::new(0.0,  1.0, 0.0),
                                       winWidth, winHeight );

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
                 changedProp:       ChangedProperties::new(),


                 wallTexture:       texture.clone(),
                 blockTexture:      block.clone(),
                 prog:              prog.clone(),
                 prog2:             prog2.clone(),
                 lightprog:         lightprog.clone(),
                 dirlightprog:      dirlightprog.clone(),

                 timer:             timer, }
    }

    fn create_gbuffer(&self, mut gbuffer: &mut MultiOutputFrameBuffer) {
        let pos = -self.Camera.GetPos();
        let CameraTranslationTrans = Matrix4D::Translation(&(-self.Camera.GetPos()));
        let CameraRotateTrans = Matrix4D::InitCameraTransform(&self.Camera.target, &self.Camera.up);
        let CameraTrans = CameraRotateTrans * CameraTranslationTrans;

        for obj in &self.objs {
            obj.draw(&mut gbuffer, &self.PerspectiveMatrix, &CameraTrans);
        }
    }

    fn create_lightbuffer(&self, render: &Render, mut lightbuffer: &mut SimpleFrameBuffer) {
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
                matrix:            render.orthomatrix,
                light_pos:         light.pos,
                light_color:       light.color,
                light_attenuation: light.attenuation,
                light_vector:      light.vector,
                light_range:       light.range,
                light_maxradius:   light.maxradius,
                pos_texture:       &render.pos_texture,
                norm_texture:      &render.norm_texture,
            };
            
            
            lightbuffer.draw(&render.vertex_buffer, &render.index_buffer, self.lightprog.prog_object(), &uniforms, &draw_params).unwrap();
        }

        for light in &self.dirlights {
            let uniforms = uniform! {
                matrix:            render.orthomatrix,
                light_color:       light.color,
                light_vector:      light.vector,
                pos_texture:       &render.pos_texture,
                norm_texture:      &render.norm_texture,
            };

            lightbuffer.draw(&render.vertex_buffer, &render.index_buffer, self.dirlightprog.prog_object(), &uniforms, &draw_params).unwrap();
        }
    }

    fn combine_buffers(&self, render: &Render, canvas: &mut glium::Frame) {
        let uniforms = uniform! {
            matrix:           render.orthomatrix,
            decal_texture:    &render.text_texture,
            lighting_texture: &render.light_texture
        };

        canvas.draw(&render.vertex_buffer, &render.index_buffer, self.prog2.prog_object(), &uniforms, &Default::default()).unwrap();
    }

    pub fn set_prop(&mut self, newProp: &ChangedProperties) {
        self.changedProp.backgroundLightColor = newProp.backgroundLightColor;
        self.changedProp.lightColor = newProp.lightColor;
    }

    pub fn draw(&self, display: &GlutinFacade, mut render: &mut Render, mut canvas: &mut glium::Frame) {
        let mut gbuffer = render.get_gbuffer(display);
        let mut light_buffer = render.get_lightbuffer(display);
        let blc = self.changedProp.backgroundLightColor;

        gbuffer.clear_color_and_depth((0.0, 0.7, 0.933, 0.0), 1.0);
        light_buffer.clear_color_and_depth((blc.x, blc.y, blc.z, 0.0), 1.0);
        
        self.create_gbuffer(&mut gbuffer);
        self.create_lightbuffer(render, &mut light_buffer);
        self.combine_buffers(render, &mut canvas);
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
        self.lights[top].set_pos(self.Camera.GetPos().as_arr()); 
        self.lights[top].set_color(self.changedProp.lightColor.as_arr());
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
        self.Viewer.update(t);

        //println!("{}", 1.0/t);

        //or obj in &self.objs {
        //   obj.init();
        // 

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

       //for obj in &self.objs {
       //    obj.update(t);
       //}

       //for i in (0..self.objs.len()) {
       //    let obj_i = &self.objs[i];

       //    for j in (0..self.objs.len()) {
       //        if ( i == j ) { continue; }
       //        let obj_j = &self.objs[j];

       //        if obj_i.intersect_with(obj_j) {
       //            obj_i.collision(&obj_j);
       //        }
       //    }
       //}

       //for obj in &self.objs {
       //    obj.rollback();
       //    obj.update(t);
       //}

        self.timer = new_timer;
    }
}