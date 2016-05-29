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

pub struct Engine{
    camera: Camera,
    display: GlutinFacade,
    program: Program,
    pos_buffers: Vec<VertexBuffer<vertex::Vertex>>,
    normal_buffers: Vec<VertexBuffer<vertex::Normal>>,
    index_buffers: Vec<IndexBuffer<u16>>,
}

impl Engine{
    pub fn new(display: GlutinFacade, program: Program) -> Engine{
        Engine{
            display: display,
            program: program,
            camera: Camera::new(),
            pos_buffers: Vec::new(),
            normal_buffers: Vec::new(),
            index_buffers: Vec::new(),
        }
    }

    pub fn init(&mut self){
        self.pos_buffers.push(glium::VertexBuffer::new(&self.display,&vertex::VERTICES).unwrap());
        self.normal_buffers.push(glium::VertexBuffer::new(&self.display,&vertex::NORMALS).unwrap());
        self.index_buffers.push(glium::IndexBuffer::new(&self.display,glium::index::PrimitiveType::TrianglesList, &vertex::INDICES).unwrap());
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

        target.clear_color_and_depth((0.0, 1.0, 0.5, 1.0), 1.0);
        self.camera.set_pos(&[10.0,0.0,2.0]);
        let view = self.camera.get_view_matrix(&[-0.000001, 0.0, 0.0]);
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

        target.draw((&self.pos_buffers[0],&self.normal_buffers[0]),&self.index_buffers[0],&self.program,&uniforms,&params).unwrap();
        target.finish().unwrap();
    }
}
