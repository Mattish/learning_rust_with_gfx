#[macro_use]
extern crate glium;
extern crate time;

mod vertex;
mod triangle;
mod shaders;

use glium::DisplayBuild;
use glium::Surface;
use time::{Duration, PreciseTime};

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let mut frame_count = 0i32;
    let mut last = PreciseTime::now();

    let triangle = triangle::Triangle::new();
    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle.shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, shaders::FRAGMENT_SHADER_SRC, None).unwrap();

    let mut t = 0.25f32;

    loop {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }
        
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
        frame_count += 1;
        if last.to(PreciseTime::now()) > Duration::seconds(1) {
            println!("fps:{}", frame_count);
            last = PreciseTime::now();
            frame_count = 0i32;
        }
    }
}
