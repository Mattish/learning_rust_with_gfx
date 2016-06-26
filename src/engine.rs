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
use image;
use std::io::Cursor;
use time::{Duration, PreciseTime};
use entity::Entity;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Engine {
    camera: Camera,
    display: GlutinFacade,
    program: Program,
    buffer_store: BufferStore,
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

        let model_vertices = models::obj_loader::load_obj_vertices("teapot.obj");
        let mut indices = Vec::new();
        for i in 0u16..model_vertices.len() as u16 {
            indices.push(i);
        }

        self.buffer_store.load_model(&self.display, "cube", &model_vertices, &indices);

        let expected_width = 20;
        let expected_height = expected_width;

        let image = image::load(Cursor::new(&include_bytes!("../1368397855550.jpg")[..]),image::JPEG).unwrap()
                         .resize_exact(expected_width,expected_height,image::FilterType::Nearest)
                         .to_rgba();
        

        let higher = (expected_width / 2) as i32;
        let lower = 0 - higher;
        
        for x in lower..higher {
            for z in lower..higher {
                let new_box = self.new_entity();
                let pixel = image.get_pixel((x+higher) as u32,(z+higher) as u32);
                new_box.borrow_mut().set_pos(x as f32, 0.0001,z as f32);
            }
        }
        let finish = start.to(PreciseTime::now());
        println!("init end took:{}ms.", finish.num_milliseconds());
        self.camera.set(2.0, 2.0, 2.0);
    }

    pub fn update_and_draw(&mut self) -> bool{
        self.frame_count += 1;

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
        if !self.update(frame_took_delta){
            return false
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
        self.frame_step_total = self.frame_step_total + frame_step_w_delta;
        let frame_count_cos = self.frame_step_total.cos();
        let frame_count_sin = self.frame_step_total.sin();
        let frame_count_sin_off = (self.frame_step_total / 1.3).sin();

        self.camera.set(frame_count_cos as f32 * 100.0,
                        frame_count_sin as f32 * 100.0,
                        frame_count_sin_off as f32 * 100.0);

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return false,
                _ => return true,
            }
        }

        for i in 0..self.entities.len() {
            let mut e = self.entities[i].borrow_mut();
            if !e.dirty{
                continue;
            }

            self.buffer_store.update_attr(e.buffer_info.index,e.buffer_info.buffer_num,
                vertex::Attr { attr: [e.pos[0], e.pos[1], e.pos[2]],scale:0.85,colour:[1.0,0.0,0.0] });
            e.clean();
        }

        self.buffer_store.do_attr_updates();
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

        self.buffer_store.draw(&mut target, &self.program, &uniforms, &params, "cube");
        target.finish().unwrap();

        self.frame_count = self.frame_count + 1;
    }


    pub fn new_entity(&mut self) -> Rc<RefCell<Entity>>{
        let attrs = [vertex::Attr { attr: [0.0, 0.0, 0.0],scale:0.85,colour:[1.0,0.0,0.0] }];
        let buffer_store_info = self.buffer_store.input_attr_range(&self.display, &attrs, "cube");
        let ent = Entity::new("cube".to_string(),buffer_store_info.buffer_num,buffer_store_info.start_index);
        let rc = Rc::new(RefCell::new(ent));
        let ret_rc = rc.clone();
        self.entities.push(rc);
        ret_rc
    }           
}
