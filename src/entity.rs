pub struct Entity{
    pub model_name: String,
    pub pos: [f32;3],
    pub dirty: bool,
}

impl Entity{
    pub fn new(model_name: String) -> Entity{
        Entity{
            model_name: model_name,
            pos: [0.0,0.0,0.0],
            dirty: false,
        }
    }

    pub fn clean(&mut self){
        self.dirty = false;
    }

    pub fn set_pos(&mut self, x: f32, y: f32, z: f32){
        self.pos[0] = x;
        self.pos[1] = y;
        self.pos[2] = z;
        self.dirty = true;
    }
}