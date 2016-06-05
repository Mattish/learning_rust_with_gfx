use time::{Duration, PreciseTime};
use glium;
use glium::Program;
use glium::Surface;
use glium::backend::glutin_backend::GlutinFacade;
use camera::Camera;
use camera;
use vertex;
use draw_parameters;
use buffer_store::BufferStore;
use models;

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
        println!("init start.");
        let start = PreciseTime::now();
        self.buffer_store.load_model(&self.display,&models::cube::VERTICES,&models::cube::INDICES);
        let mut attrs :[vertex::Attr;100] = [vertex::Attr{attr:[0.0,0.0,0.0]};100];
        let mut counter = 0;
        for x in -5..5 {
            for z in -5..5 {
                attrs[counter] = vertex::Attr{attr:[x as f32,0.0001,z as f32]};
                counter = counter + 1;
            }
        }
        self.buffer_store.input_attr_range(&self.display, &attrs);
        let finish = start.to(PreciseTime::now());
        println!("init end took:{}ms.",finish.num_milliseconds());
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
        self.camera.set_pos(&[20.0,20.0,20.0]);
        let view = self.camera.get_view_matrix([-0.000001, 0.0, 0.0]);
        let (width, height) = target.get_dimensions();
        let perspective = camera::get_perspectivei(height, width);
        let uniforms = uniform! {
            u_light: [1.0, -0.4, 0.9f32],
            perspective: perspective,
            view: view
        };
        let params = draw_parameters::get();

        self.buffer_store.draw(&mut target,&self.program,&uniforms,&params);
        target.finish().unwrap();
    }
}
