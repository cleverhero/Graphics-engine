extern crate glium;

use math::Vertex;
use math::Vector3D;
use math::Matrix4D;
use models::CModel;
use world::CWorld;
use render::Render;

use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::framebuffer::MultiOutputFrameBuffer;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::Texture2d;
use glium::texture::DepthTexture2d;
use glium::texture::UncompressedFloatFormat::F32F32F32F32;
use glium::texture::DepthFormat;
use glium::texture::MipmapsOption::NoMipmap;

struct CWindow {
	Facade: GlutinFacade,
	Width: u32,
	Height: u32,
}

pub struct CGame {
	//Interface:
	Window: CWindow,
}

impl CGame {
	pub fn new(display: GlutinFacade, width: u32, height: u32) -> CGame {
		CGame {
			Window: CWindow {
				Facade: display,
				Height: height,
				Width:  width
			}, 
		}
	}

	pub fn start_loop(&self) {
		let width = self.Window.Width;
		let height = self.Window.Height;

		self.Window.Facade.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Grab);
    	self.Window.Facade.get_window().unwrap().set_cursor_position((width / 2) as i32, (height / 2) as i32);

    	let mut world = CWorld::new(&self.Window.Facade, width, height);
    	let mut render = Render::new(&self.Window.Facade, width, height);

		loop {
        	world.draw(&self.Window.Facade, &mut render);
			
        	for ev in self.Window.Facade.poll_events() {
        	    match ev {
        	        glium::glutin::Event::Closed => return,
        	        _ => world.checkEvents(&ev, &self.Window.Facade),
        	    }
	
        	}
        	self.Window.Facade.get_window().unwrap().set_cursor_position((width / 2) as i32, (height / 2) as i32);
        	world.update();
    	}
	}
}