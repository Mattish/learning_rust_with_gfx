#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate vecmath;

mod vertex;
mod triangle;
mod shaders;
mod camera;
mod draw_parameters;
mod engine;
mod buffer_store;
mod wrappers;
mod models;

use glium::DisplayBuild;
use glium::Surface;
use time::{Duration, PreciseTime};
//use std::io::Cursor;

fn main() {
    let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let mut frame_count = 0i32;
    let mut last = PreciseTime::now();

    let program = glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, shaders::FRAGMENT_SHADER_SRC,None).unwrap();
    let mut engine = engine::Engine::new(display,program);
    engine.init();
    //let image = image::load(Cursor::new(&include_bytes!("../1368397855550.jpg")[..]),image::JPEG).unwrap().to_rgba();
    //let image_dimensions = image.dimensions();
    //let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    loop {

        if engine.update(){
            return
        }

        engine.draw();

        frame_count += 1;
        if last.to(PreciseTime::now()) > Duration::seconds(1) {
            println!("fps:{}", frame_count);
            last = PreciseTime::now();
            frame_count = 0i32;
        }
    }
}
