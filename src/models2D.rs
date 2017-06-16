extern crate cgmath;
extern crate glium;
extern crate unicode_normalization;
extern crate arrayvec;
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
use std;
use glium::texture::Texture2d;
use render::Render;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use std::borrow::Cow;
use self::unicode_normalization::UnicodeNormalization;


fn layout_paragraph<'a>(font: &'a Font, scale: Scale, text: &str) -> (Vec<PositionedGlyph<'a>>, f32) {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);

    let mut caret = point(0.0, v_metrics.ascent);

    for c in text.nfc() {
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };

        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
	(result, caret.x)
}



pub struct Rect {
	prog: Rc<CProgram>,

	prog_text: CProgram,

	pub color: Vector3D,

	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new(display: &GlutinFacade, prog: &Rc<CProgram>, x: f32, y: f32, width: f32, height: f32) -> Rect {
		Rect {
			x: x,
			y: y,

			width:  width,
			height: height,

			prog: prog.clone(),
			prog_text: CProgram::load(display, "Shaders/TextV.vs", "Shaders/TextF.fs"),

			color: Vector3D::new(0.3, 0.3, 0.3),

		}
	}

	pub fn set_color(&mut self, new_color: Vector3D) {
		self.color = new_color;
	}

	pub fn is_inside(&self, x: f32, y: f32) -> bool {
		let x0 = self.x;
		let y0 = self.y;
		let x1 = self.x + self.width;
		let y1 = self.y + self.height;

		if (x >= x0 && x <= x1 && y >= y0 && y <= y1) {
			return true;
		}

		return false;
	}

	fn create_buffers(&self, display: &GlutinFacade) -> (glium::VertexBuffer<VertexPT>, glium::IndexBuffer<u16>) {
		let x0 = self.x;
		let y0 = self.y;
		let x1 = self.x + self.width;
		let y1 = self.y + self.height;

		let Verteces = [
			VertexPT{ position: [x0, y1, 0.0], tex_coord: [0.0, 0.0] },
        	VertexPT{ position: [x1, y1, 0.0], tex_coord: [1.0, 0.0] },
        	VertexPT{ position: [x1, y0, 0.0], tex_coord: [1.0, 1.0] },
        	VertexPT{ position: [x0, y0, 0.0], tex_coord: [0.0, 1.0] },
		];

		let vertex_buffer = glium::VertexBuffer::new(display, &Verteces).unwrap();
    	let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[1, 0, 2, 0, 2, 3u16]
    		).unwrap();

    	(vertex_buffer, index_buffer)
	}

	pub fn draw(&self, display: &GlutinFacade, render: &mut Render, canvas: &mut glium::Frame) {
		let (vertex_buffer, index_buffer) = self.create_buffers(display);

    	let uniforms = uniform! {
            matrix: render.orthomatrix,
            color:  self.color.as_arr(),
        };

		canvas.draw(&vertex_buffer, &index_buffer, self.prog.prog_object(), &uniforms, &Default::default()).unwrap();
	}

	pub fn draw_text(&self, display: &GlutinFacade, render: &mut Render, canvas: &mut glium::Frame, font: &Font, text: String) {
		let dpi_factor = display.get_window().unwrap().hidpi_factor();

		let (cache_width, cache_height) = ((self.width as u32), (self.height as u32));
		let mut cache = Cache::new(cache_width, cache_height, 0.1, 0.1);

		let cache_tex = glium::texture::Texture2d::with_format(
        	display,
        	glium::texture::RawImage2d {
        	    data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
        	    width: cache_width,
        	    height: cache_height,
        	    format: glium::texture::ClientFormat::U8
        	},
        	glium::texture::UncompressedFloatFormat::U8,
			glium::texture::MipmapsOption::NoMipmap
		).unwrap();

		let (vertex_buffer, index_buffer) = self.create_buffers(display);


        let (glyphs, text_width) = layout_paragraph(&font, Scale::uniform(80.0), &text);

        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.clone());
        }

        cache.cache_queued(|rect, data| {
            cache_tex.main_level().write(glium::Rect {
                left: rect.min.x + (cache_width - text_width as u32) / 2,
                bottom: rect.min.y + (cache_height - rect.height()) / 2,
                width: rect.width(),
                height: rect.height()
            }, glium::texture::RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: glium::texture::ClientFormat::U8
            });
		}).unwrap();

        let color = Vector3D::new(0.0, 0.0, 0.0); 
    	let uniforms = uniform! {
            matrix: render.orthomatrix,
            tex:    cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            color:  color.as_arr(),
        };

        let draw_params = &glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

		canvas.draw(&vertex_buffer, 
					//glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), 
					&index_buffer,
					self.prog_text.prog_object(), &uniforms, &draw_params).unwrap();
	}
}