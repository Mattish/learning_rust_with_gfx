#[macro_use]
extern crate glium;
extern crate time;
extern crate image;
extern crate vecmath;
extern crate obj;

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
    let mut frame_delta_total = 0.0f64;
    let mut last = PreciseTime::now();

    let program = glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, shaders::FRAGMENT_SHADER_SRC,None).unwrap();
    let mut engine = engine::Engine::new(display,program);
    engine.init();
    //let image = image::load(Cursor::new(&include_bytes!("../1368397855550.jpg")[..]),image::JPEG).unwrap().to_rgba();
    //let image_dimensions = image.dimensions();
    //let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let target_frame_rate = 60.0f64;
    let frame_step = Duration::seconds(1).num_nanoseconds().unwrap() as f64 / target_frame_rate;
    let mut frame_took_delta = 0.0f64;
    loop {
        let frame_limit_last = PreciseTime::now(); 
        
        frame_count += 1;

        if last.to(PreciseTime::now()) > Duration::seconds(1) {
            println!("fps:{}", frame_count);
            println!("frame_delta_total:{}", frame_delta_total);
            last = PreciseTime::now();
            frame_count = 0i32;
            frame_delta_total = 0.0f64;
        }
        
        if engine.update(frame_took_delta){
            return
        }
        engine.draw();

        let mut frame_took = frame_limit_last.to(PreciseTime::now());
        let mut frame_took_nano = frame_took.num_nanoseconds().unwrap() as f64;        
        frame_took_delta = frame_took_nano / frame_step as f64;

        while frame_took_delta < 0.9995 {
            let nano_to_sleep = frame_step - frame_took_nano;

            std::thread::sleep(std::time::Duration::new(0, nano_to_sleep as u32));
            frame_took = frame_limit_last.to(PreciseTime::now());
            frame_took_nano = frame_took.num_nanoseconds().unwrap() as f64;
            
            frame_took_delta = frame_took_nano / frame_step as f64;

        }

        frame_delta_total += frame_took_delta;
        
    }
}
