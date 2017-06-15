extern crate cgmath;
extern crate glium;
extern crate rusttype;

use program::CProgram;
use glium::index::PrimitiveType;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{DisplayBuild, Surface, glutin};
use glutin::ElementState::Pressed;
use glutin::ElementState::Released;
use math::VertexPT;
use math::Size2;
use math::Point;
use math::Vector3D;
use std::rc::Rc;
use models2D::Rect;
use button::Button;
use trackbar::TrackBar;
use world::ChangedProperties;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use render::Render;

#[derive(Debug, Clone, Copy)]
pub enum ControllEvent {
    Click,
    Changed,
}

pub trait Controller {
    fn tap(&mut self, x: f32, y: f32);
    fn untap(&mut self);
    fn moveTo(&mut self, x: f32, y: f32);

    fn get_events(&mut self) -> Vec<ControllEvent>;

    fn draw(&mut self, display: &GlutinFacade, render: &mut Render, canvas: &mut glium::Frame);
    fn setValue(&mut self, value: f32);
    fn getValue(&mut self) -> f32;
}

fn new_font() -> rusttype::Font<'static> {
    let font_data = include_bytes!("Arial Unicode.ttf");
    FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap()
}

pub struct Interface {
    pub elements:  Vec<Box<Controller>>,

    cursor:      Point,

    pub changedProp:   ChangedProperties,
    pub font: Font<'static>,
}

impl Interface {
    pub fn new(display: &GlutinFacade, winSize: Size2) -> Interface {
        let prog = Rc::new(CProgram::load(display, "Shaders/2DV.vs", "Shaders/2DF.fs"));

        let mut bottonDefault = Box::new( Button::new(&prog, 20.0, 450.0, 60.0, 20.0) );
        let mut backgroundLightR = Box::new( TrackBar::new(&prog, 10.0,  500.0, 60.0, 20.0) );
        let mut backgroundLightG = Box::new( TrackBar::new(&prog, 80.0,  500.0, 60.0, 20.0) );
        let mut backgroundLightB = Box::new( TrackBar::new(&prog, 150.0, 500.0, 60.0, 20.0) );
        backgroundLightR.setValue(0.05);
        backgroundLightG.setValue(0.05);
        backgroundLightB.setValue(0.05);

        let mut LightR = Box::new( TrackBar::new(&prog, 10.0,  550.0, 60.0, 20.0) );
        let mut LightG = Box::new( TrackBar::new(&prog, 80.0,  550.0, 60.0, 20.0) );
        let mut LightB = Box::new( TrackBar::new(&prog, 150.0, 550.0, 60.0, 20.0) );
        LightR.setValue(100.0);
        LightG.setValue(0.0);
        LightB.setValue(0.0);

        let mut bottonSave = Box::new( Button::new(&prog, 130.0, 450.0, 60.0, 20.0) );

        Interface {
            elements: vec![ bottonDefault,
                            backgroundLightR,
                            backgroundLightG,
                            backgroundLightB,
                            LightR,
                            LightG,
                            LightB,
                            bottonSave, ],

            cursor:      Point::new(0.0, 0.0),
            changedProp: ChangedProperties::new(),
            font:        new_font()
        }
    }

    pub fn draw(&mut self, display: &GlutinFacade, render: &mut Render, mut canvas: &mut glium::Frame) {
        for element in &mut self.elements {
            element.draw(display, render, canvas);
        }
    }

    pub fn checkEvents(&mut self, event: &glium::glutin::Event, display: &GlutinFacade) {
        match *event {
            glutin::Event::MouseMoved(x, y) => {
                self.cursor.x = x as f32;
                self.cursor.y = 600.0 - y as f32;
                for element in &mut self.elements {
                    element.moveTo(self.cursor.x, self.cursor.y);
                }
            }
            glutin::Event::MouseInput(state, glutin::MouseButton::Left) => {
                for element in &mut self.elements {
                    match state {
                        Pressed => {
                            element.tap(self.cursor.x, self.cursor.y);
                        },
                        Release => {
                            element.untap();
                        },
                    }
                }
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {
        for event in self.elements[0].get_events() {
            match event {
                ControllEvent::Click => { self.OnClickDefault(); },
                _ => { }
            }
        }

        for event in self.elements[7].get_events() {
            match event {
                ControllEvent::Click => { self.OnClickSave(); },
                _ => { }
            }
        }
    }

    pub fn OnClickDefault(&mut self) {
        self.elements[1].setValue(0.5);
        self.elements[2].setValue(0.5);
        self.elements[3].setValue(0.5);

        self.elements[4].setValue(100.0);
        self.elements[5].setValue(0.0);
        self.elements[6].setValue(0.0);
    }

    pub fn OnClickSave(&mut self) {
        let mut r = self.elements[1].getValue() / 100.0;
        let mut g = self.elements[2].getValue() / 100.0;
        let mut b = self.elements[3].getValue() / 100.0;
        self.changedProp.backgroundLightColor = Vector3D::new(r, g, b);

        r = self.elements[4].getValue() / 100.0;
        g = self.elements[5].getValue() / 100.0;
        b = self.elements[6].getValue() / 100.0;
        self.changedProp.lightColor = Vector3D::new(r, g, b);
    }
}
