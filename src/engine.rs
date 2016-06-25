use time::PreciseTime;
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
use std;
use std::f64;
use rand::Rng;
use rand;

pub struct Engine {
    camera: Camera,
    display: GlutinFacade,
    program: Program,
    buffer_store: BufferStore,
    frame_count: i32,
    frame_step_total: f64,
}

impl Engine {
    pub fn new(display: GlutinFacade, program: Program) -> Engine {
        Engine {
            display: display,
            program: program,
            camera: Camera::new(),
            buffer_store: BufferStore::new(),
            frame_count: 0,
            frame_step_total: 0.0,
        }
    }

    pub fn init(&mut self) {
        println!("init start.");
        let start = PreciseTime::now();

        let model_vertices = models::obj_loader::load_obj_vertices("cube.obj");
        let mut indices = Vec::new();
        for i in 0u16..model_vertices.len() as u16 {
            indices.push(i);
        }

        self.buffer_store.load_model(&self.display, "cube", &model_vertices, &indices);

        let mut rng = rand::thread_rng();
        let mut attrs_one = Vec::new();
        let mut attrs_two = Vec::new();
        let mut counter = 0;
        for x in -250..250 {
            for z in -250..250 {
                attrs_one.push(vertex::Attr { attr: [x as f32, 0.0001, z as f32],scale:0.65,colour:[rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()] });
                attrs_two.push(vertex::Attr { attr: [(x as f32) + 0.5, 1.0001, (z as f32) + 0.5],scale:0.65,colour:[rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()] });
                counter = counter + 1;
            }
        }
        self.buffer_store.input_attr_range(&self.display, &attrs_one, "cube");
        self.buffer_store.input_attr_range(&self.display, &attrs_two, "cube");
        self.buffer_store.update_attr(4,vertex::Attr { attr: [2.0, 2.0, 2.0],scale:2.0,colour:[1.0,1.0,0.0] });
        let finish = start.to(PreciseTime::now());
        println!("init end took:{}ms.", finish.num_milliseconds());
        self.camera.set(2.0, 2.0, 2.0);
    }

    pub fn update(&mut self, delta: f64) -> bool {

        let frame_step: f64 = (std::f64::consts::PI * 1.0) / 600.0;
        let frame_step_w_delta = frame_step * delta;
        self.frame_step_total = self.frame_step_total + frame_step_w_delta;
        let frame_count_cos = self.frame_step_total.cos();
        let frame_count_sin = self.frame_step_total.sin();
        let frame_count_sin_off = (self.frame_step_total / 1.3).sin();

        self.camera.set(frame_count_cos as f32 * 20.0,
                        frame_count_sin as f32 * 20.0,
                        frame_count_sin_off as f32 * 20.0);

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return true,
                _ => return false,
            }
        }
        return false;
    }

    pub fn draw(&mut self) {
        let mut target = self.display.draw();

        target.clear_color_and_depth((1.0, 1.0, 1.0, 0.0), 1.0);

        let view = self.camera.get_view_matrix([0.0, 0.0, 0.0]);
        let (width, height) = target.get_dimensions();
        let perspective = camera::get_perspectivei(height, width);
        let uniforms = uniform! {
            u_light: [1.0, -0.4, 0.9f32],
            perspective: perspective,
            view: view
        };
        let params = draw_parameters::get();

        self.buffer_store.draw(&mut target, &self.program, &uniforms, &params, "cube");
        target.finish().unwrap();

        self.frame_count = self.frame_count + 1;
    }
}
