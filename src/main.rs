#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate vecmath;

mod vertex;
mod shaders;
mod camera;
mod draw_parameters;
mod engine;
mod buffer_store;
mod wrappers;
mod models;
mod entity;

use glium::DisplayBuild;
use std::rc::{Rc,Weak};
//use std::io::Cursor;

fn main() {
    let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let program = glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, shaders::FRAGMENT_SHADER_SRC,None).unwrap();
    let mut engine = engine::Engine::new(display,program);
    engine.init();
    //let image_dimensions = image.dimensions();
    //let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    

    let some_ent = engine.new_entity();
    //println!("some_ent.weak:{}",Rc::weak_count(some_ent));
    loop {
        if !engine.update_and_draw(){
            return
        }
    }
}
