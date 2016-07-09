pub struct Entity {
    pub model_id: usize,
    pub pos: [f32; 3],
    pub colour: [f32; 3],
}

impl Entity {
    pub fn new(model_id: usize) -> Entity {
        Entity {
            model_id: model_id,
            pos: [0.0, 0.0, 0.0],
            colour: [0.0, 1.0, 0.0]
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.pos[0] = x;
        self.pos[1] = y;
        self.pos[2] = z;
    }

    /// Change RGB values of the entity. Range is from 0.0 -> 1.0.
    pub fn set_colour(&mut self, r: f32, g: f32, b: f32) {
        self.colour[0] = r;
        self.colour[1] = g;
        self.colour[2] = b;
    }
}