#[macro_use]
extern crate glium;
extern crate time;
extern crate image;

mod vertex;
mod triangle;
mod shaders;
mod camera;

use glium::DisplayBuild;
use glium::Surface;
use time::{Duration, PreciseTime};
//use std::io::Cursor;

fn main() {
    let mut camera = camera::Camera::new();
    camera.set_pos(&[2.0, -1.0, 1.0]);
    let mut camera2 = camera::Camera::new();
    camera2.set_pos(&[2.0, -0.5, 1.0]);
    let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let mut frame_count = 0i32;
    let mut last = PreciseTime::now();

    let teapot_pos = glium::VertexBuffer::new(&display,&vertex::VERTICES).unwrap();
    let teapot_normals = glium::VertexBuffer::new(&display,&vertex::NORMALS).unwrap();
    let teapot_indices = glium::IndexBuffer::new(&display,glium::index::PrimitiveType::TrianglesList, &vertex::INDICES).unwrap();

    let program = glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, shaders::FRAGMENT_SHADER_SRC,None).unwrap();

    //let image = image::load(Cursor::new(&include_bytes!("../1368397855550.jpg")[..]),image::JPEG).unwrap().to_rgba();
    //let image_dimensions = image.dimensions();
    //let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    loop {

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
        let mut target = display.draw();
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let mut view = camera.get_view_matrix(&[-2.0, 1.0, 1.0]);
        if frame_count > 1000{
            view = camera2.get_view_matrix(&[-2.0, 1.0, 1.0]);
        }
        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 2.0, 1.0f32]
            ],
            u_light: [1.0, -0.4, 0.9f32],
            perspective: perspective,
            view: view
        };
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw((&teapot_pos,&teapot_normals),&teapot_indices,&program,&uniforms,&params).unwrap();
        target.finish().unwrap();
        frame_count += 1;
        if last.to(PreciseTime::now()) > Duration::seconds(1) {
            println!("fps:{}", frame_count);
            last = PreciseTime::now();
            frame_count = 0i32;
        }
    }
}
