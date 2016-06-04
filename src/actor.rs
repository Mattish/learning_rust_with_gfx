use glium;

pub trait DrawableActor{
    fn draw(&self, target: glium::Frame);
    fn update(&self);
}

pub struct SimpleActor{

}

impl DrawableActor for SimpleActor{
    fn draw(&self,  target: glium::Frame){

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
    }

    fn update(&self){

    }
}
