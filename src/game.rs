extern crate glium;

use math::Vertex;
use math::Vector3D;
use math::Matrix4D;
use models::CModel;
use world::CWorld;
use render_object::Render;

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

    	let mut World = CWorld::new(&self.Window.Facade, width, height);

    	let pos_texture = Texture2d::empty_with_format(&self.Window.Facade, F32F32F32F32, NoMipmap, width, height).unwrap();
    	let norm_texture = Texture2d::empty_with_format(&self.Window.Facade, F32F32F32F32, NoMipmap, width, height).unwrap();
    	let text_texture = Texture2d::empty_with_format(&self.Window.Facade, F32F32F32F32, NoMipmap, width, height).unwrap();

		let depthtexture = DepthTexture2d::empty_with_format(&self.Window.Facade, DepthFormat::F32, NoMipmap, width, height).unwrap();

    	let output = &[("pos_texture", &pos_texture), ("norm_texture", &norm_texture), ("text_texture", &text_texture)];
    	let mut frame_buffer = MultiOutputFrameBuffer::with_depth_buffer(&self.Window.Facade, output.iter().cloned(), &depthtexture).unwrap();
    	
    	let light_texture = Texture2d::empty_with_format(&self.Window.Facade, F32F32F32F32, NoMipmap, width, height).unwrap();
    	let mut light_buffer = SimpleFrameBuffer::with_depth_buffer(&self.Window.Facade, &light_texture, &depthtexture).unwrap();

    	let mut render = Render{ pos_texture:       &pos_texture,
							 	 norm_texture:      &norm_texture,
							 	 text_texture:      &text_texture,
	 	 
							 	 gbuffer:           frame_buffer,
	 	 
							 	 depthtexture:      &depthtexture,
							 	 light_texture:     &light_texture,
							 	 light_buffer:      light_buffer, };

		loop {
        	World.draw(&self.Window.Facade, &mut render);
			
        	for ev in self.Window.Facade.poll_events() {
        	    match ev {
        	        glium::glutin::Event::Closed => return,
        	        _ => World.checkEvents(&ev, &self.Window.Facade),
        	    }
	
        	}

        World.update();
    }
	}
}