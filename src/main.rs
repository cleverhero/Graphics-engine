#[macro_use]
extern crate glium;
extern crate image;
extern crate glutin;
extern crate cgmath;
extern crate rusttype;

mod math;
mod geometry;
mod light;
mod pool;
mod shell;
mod models;
mod render_object;
mod physical_object;
mod game_object;
mod world;
mod camera;
mod texture;
mod program;
mod game;
mod viewer;
mod text;
mod render;
mod gui;

use math::Vertex;
use math::Vector3D;
use math::Matrix4D;
use models::CModel;
use world::CWorld;
use game::CGame;

use glium::{DisplayBuild, Surface};

const  DEFOULD_WINDOW_WIDTH:  u32  = 800;
const  DEFOULD_WINDOW_HEIGHT: u32  = 600;

fn main() {
    let display = glium::glutin::WindowBuilder::new().
                                    with_depth_buffer(24).
                                    with_dimensions(DEFOULD_WINDOW_WIDTH, DEFOULD_WINDOW_HEIGHT).
                                    build_glium().unwrap();

    let mut game = CGame::new(display, DEFOULD_WINDOW_WIDTH, DEFOULD_WINDOW_HEIGHT);
    game.start_loop();
}