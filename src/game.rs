extern crate glium;

use math::Vertex;
use math::Vector3D;
use math::Matrix4D;
use math::Size2;
use models::CModel;
use world::CWorld;
use render::Render;
use gui::Interface;


use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::framebuffer::MultiOutputFrameBuffer;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use glium::texture::DepthTexture2d;
use glium::texture::UncompressedFloatFormat::F32F32F32F32;
use glium::texture::DepthFormat;
use glium::texture::MipmapsOption::NoMipmap;
use glium::glutin;

struct CWindow {
	Facade: GlutinFacade,
	Width: u32,
	Height: u32,
}

#[derive(Debug)]
enum GameState {
    World,
    Interface,
}

pub struct CGame {
	Window: CWindow,
	State: GameState
}

impl CGame {
	pub fn new(display: GlutinFacade, width: u32, height: u32) -> CGame {
		CGame {
			Window: CWindow {
				Facade: display,
				Height: height,
				Width:  width
			}, 
			State: GameState::World,
		}
	}

	pub fn start_loop(&mut self) {
		let width = self.Window.Width;
		let height = self.Window.Height;

		self.Window.Facade.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Grab);
    	self.Window.Facade.get_window().unwrap().set_cursor_position((width / 2) as i32, (height / 2) as i32);

    	let mut world = CWorld::new(&self.Window.Facade, width, height);
    	let mut render = Render::new(&self.Window.Facade, width, height);
    	let mut interface = Interface::new(&self.Window.Facade, Size2{w: width, h: height});

		loop {
			let mut canvas = self.Window.Facade.draw();
			canvas.clear_color(0.0, 0.0, 0.0, 0.0);

        	world.draw(&self.Window.Facade, &mut render, &mut canvas);
        	interface.draw(&self.Window.Facade, &mut canvas);
			
        	for ev in self.Window.Facade.poll_events() {
        	    match ev {
        	        glium::glutin::Event::Closed => return,
        	        glutin::Event::KeyboardInput(glutin::ElementState::Pressed,  _, Some(glutin::VirtualKeyCode::LControl)) => {
						self.State = GameState::Interface;
            		},
            		glutin::Event::KeyboardInput(glutin::ElementState::Released,  _, Some(glutin::VirtualKeyCode::LControl)) => {
						self.State = GameState::World;
            		},
        	        _ => {
        	        	match self.State {
        	        		GameState::World => {
								world.checkEvents(&ev, &self.Window.Facade);
        					},
        					GameState::Interface => { 
        						interface.checkEvents(&ev, &self.Window.Facade);
        					}
        				}
        	        } 
        	    }
	
        	}

        	match self.State {
        		GameState::World => {
        			self.Window.Facade.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Grab);
        			self.Window.Facade.get_window().unwrap().set_cursor_position((width / 2) as i32, (height / 2) as i32);
        		},
        		GameState::Interface => {
        			self.Window.Facade.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Normal);
        		}
        	}

            world.set_prop(&interface.changedProp);
			world.update();
            interface.update();
			canvas.finish().unwrap();
    	}
	}
}