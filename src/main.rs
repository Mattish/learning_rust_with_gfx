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
mod entity_model_packer;

use glium::DisplayBuild;
use std::io::Cursor;

fn main() {
    let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let program = glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, shaders::FRAGMENT_SHADER_SRC,None).unwrap();
    let mut engine = engine::Engine::new(display,program);
    engine.init();
    //let image_dimensions = image.dimensions();
    //let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    
    let expected_width = 50;
    let expected_height = expected_width;

    let image = image::load(Cursor::new(&include_bytes!("../1368397855550.jpg")[..]),image::JPEG).unwrap()
                        .resize_exact(expected_width,expected_height,image::FilterType::Nearest)
                        .to_rgba();
    

    let higher = (expected_width / 2) as i32;
    let lower = 0 - higher;
    for x in lower..higher {
        for z in lower..higher {
            let mut model_name = "cube";
            if z % 2 == 0{
                model_name = "teapot"; 
            }
            let new_box = engine.new_entity(model_name);
            let pixel = image.get_pixel((x+higher) as u32,(z+higher) as u32);
            new_box.borrow_mut().set_pos(x as f32, 0.0001,z as f32);
        }
    }
    loop {
        if !engine.update_and_draw(){
            return
        }
    }
}
