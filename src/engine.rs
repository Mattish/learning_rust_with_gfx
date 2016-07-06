use glium;
use glium::Program;
use glium::Surface;
use glium::backend::glutin_backend::GlutinFacade;
use camera::Camera;
use camera;
use draw_parameters;
use buffer_store::BufferStore;
use models;
use std;
use std::f64;
use time::{Duration, PreciseTime};
use entity::Entity;
use std::cell::RefCell;
use std::rc::Rc;
use entity_model_packer;

pub struct Engine {
    camera: Camera,
    display: GlutinFacade,
    program: Program,
    buffer_store: BufferStore,
    frame_number: i32,
    frame_count: i32,
    frame_count_timer: PreciseTime,
    frame_step: f64,
    frame_step_total: f64,
    last_frame_time: PreciseTime,
    entities: Vec<Rc<RefCell<Entity>>>,
}

impl Engine {
    pub fn new(display: GlutinFacade, program: Program) -> Engine {
        Engine {
            display: display,
            program: program,
            camera: Camera::new(),
            buffer_store: BufferStore::new(),
            frame_number: 0,
            frame_count: 0,
            frame_count_timer: PreciseTime::now(),
            frame_step: Duration::seconds(1).num_nanoseconds().unwrap() as f64 / 60.0,
            frame_step_total: 0.0,
            last_frame_time: PreciseTime::now(),
            entities: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        println!("init start.");
        let start = PreciseTime::now();

        let mut model_vertices = models::obj_loader::load_obj_vertices("cube.obj");
        let mut indices = Vec::new();
        for i in 0..model_vertices.len() as u16 {
            indices.push(i);
        }
        self.buffer_store.load_model(&self.display, "cube", &model_vertices, &indices);
        let total_from_teapot = model_vertices.len() as u16;
        model_vertices = models::obj_loader::load_obj_vertices("teapot.obj");
        indices = Vec::new();
        for i in 0..model_vertices.len() as u16 {
            indices.push(i + total_from_teapot);
        }
        self.buffer_store.load_model(&self.display, "teapot", &model_vertices, &indices);

        let finish = start.to(PreciseTime::now());
        println!("init end took:{}ms.", finish.num_milliseconds());
        self.camera.set(2.0, 2.0, 2.0);
    }

    pub fn update_and_draw(&mut self) -> bool {
        self.frame_count += 1;
        self.frame_number += 1;
        if self.frame_count_timer.to(PreciseTime::now()) > Duration::seconds(1) {
            println!("fps:{}", self.frame_count);
            self.frame_count_timer = PreciseTime::now();
            self.frame_count = 0i32;
        }

        let old = self.last_frame_time;
        self.last_frame_time = PreciseTime::now();
        let frame_took = old.to(self.last_frame_time);
        let frame_took_nano = frame_took.num_nanoseconds().unwrap() as f64;
        let frame_took_delta = frame_took_nano / self.frame_step as f64;
        if !self.update(frame_took_delta) {
            return false;
        }
        self.draw();

        if frame_took_delta < 0.9 {
            let nano_to_sleep = self.frame_step - frame_took_nano;
            std::thread::sleep(std::time::Duration::new(0, (nano_to_sleep / 2.0) as u32));
        }
        true
    }

    fn update(&mut self, delta: f64) -> bool {

        let frame_step: f64 = (std::f64::consts::PI * 1.0) / 1200.0;
        let frame_step_w_delta = frame_step * delta;
        self.frame_step_total += frame_step_w_delta;
        let frame_count_cos = self.frame_step_total.cos();
        let frame_count_sin = self.frame_step_total.sin();
        let frame_count_sin_off = (self.frame_step_total / 1.3).sin();

        self.camera.set(frame_count_cos as f32 * 20.0,
                        frame_count_sin as f32 * 20.0,
                        frame_count_sin_off as f32 * 20.0);

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return false,
                _ => return true,
            }
        }

        true
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();

        target.clear_color_and_depth((1.0, 1.0, 1.0, 0.0), 1.0);

        let view = self.camera.get_view_matrix([0.0, 0.0, 0.0]);
        let (width, height) = target.get_dimensions();
        let perspective = camera::get_perspectivei(height, width);
        let uniforms = uniform! {
            perspective: perspective,
            view: view
        };
        let params = draw_parameters::get();

        let ent_pack = entity_model_packer::pack(self.entities.as_mut_slice());
        if self.frame_number < 20 {
            println!("something! frame_number:{}",self.frame_number);
            self.buffer_store.input_attr_range(&self.display, ent_pack.attrs.as_slice());
        }
        self.buffer_store.draw(&mut target, &self.program, &uniforms, &params, ent_pack);
        target.finish().unwrap();
    }


    pub fn new_entity(&mut self, model_name: &str) -> Rc<RefCell<Entity>> {
        // let attrs = [vertex::Attr { attr: [0.0, 0.0, 0.0],scale:0.85,colour:[1.0,0.0,0.0] }];
        let ent = Entity::new(model_name.to_string());
        let rc = Rc::new(RefCell::new(ent));
        let ret_rc = rc.clone();
        self.entities.push(rc);
        ret_rc
    }
}
