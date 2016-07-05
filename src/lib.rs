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

pub fn run() {
    let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let program = glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, shaders::FRAGMENT_SHADER_SRC,None).unwrap();
    let mut engine = engine::Engine::new(display,program);
    engine.init();
    //let image_dimensions = image.dimensions();
    //let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    
    let expected_width = 100;
    let expected_height = expected_width;

    let image = image::load(Cursor::new(&include_bytes!("../1368397855550.jpg")[..]),image::JPEG).unwrap()
                        .resize_exact(expected_width,expected_height,image::FilterType::Nearest)
                        .to_rgba();
    

    let higher = (expected_width / 2) as i32;
    let lower = 0 - higher;
    for x in lower..higher {
        for z in lower..higher {
            let mut model_name = "cube";
            // if z % 2 == 0{
            //     model_name = "teapot"; 
            // }
            let new_box = engine.new_entity(model_name);
            let pixel = image.get_pixel((x+higher) as u32,(z+higher) as u32);
            let mut box_borrow = new_box.borrow_mut();
            box_borrow.set_pos(x as f32, 0.0001,z as f32);
            box_borrow.set_colour(normalize(pixel[0]),normalize(pixel[1]),normalize(pixel[2]));
        }
    }
    loop {
        if !engine.update_and_draw(){
            return
        }
    }
}

fn normalize(input: u8) -> f32{
    let f_input = input as f32;
    f_input / 255.0
}
