use glium;
use glium::Program;
use glium::Surface;
use glium::backend::glutin_backend::GlutinFacade;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use camera::Camera;
use std::vec::Vec;
use camera;
use vertex;
use draw_parameters;
use buffer_store;
use buffer_store::BufferStore;
//use teapot;
//use actor;

pub struct Engine{
    camera: Camera,
    display: GlutinFacade,
    program: Program,
    buffer_store: BufferStore,
}

impl Engine{
    pub fn new(display: GlutinFacade, program: Program) -> Engine{
        Engine{
            display: display,
            program: program,
            camera: Camera::new(),
            buffer_store: BufferStore::new(),
        }
    }

    pub fn init(&mut self){
        self.buffer_store.input_verticies(&self.display, &vertex::VERTICES);
        self.buffer_store.input_normals(&self.display, &vertex::NORMALS);
        self.buffer_store.input_indices(&self.display, &vertex::INDICES);
    }

    pub fn update(&self) -> bool{
        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return true,
                _ => return false,
            }
        }
        return false;
    }

    pub fn draw(&mut self){
        let mut target = self.display.draw();

        target.clear_color_and_depth((1.0, 1.0, 1.0, 0.0), 1.0);
        self.camera.set_pos(&[10.0,10.0,0.0]);
        let view = self.camera.get_view_matrix([-0.000001, 0.0, 0.0]);
        let (width, height) = target.get_dimensions();
        let perspective = camera::get_perspectivei(height, width);
        let uniforms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, -2.0, 1.0f32]
            ],
            u_light: [1.0, -0.4, 0.9f32],
            perspective: perspective,
            view: view
        };
        let params = draw_parameters::get();

        self.buffer_store.draw(&mut target,&self.program,&uniforms,&params);
        target.finish().unwrap();
    }
}
