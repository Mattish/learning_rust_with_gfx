pub struct EntityBufferInfo{
    pub model_name: String,
    pub buffer_num: usize,
    pub index: usize,
}

pub struct Entity{
    pub buffer_info: EntityBufferInfo,
    pub pos: [f32;3],
    pub dirty: bool,
}

impl Entity{
    pub fn new(model_name: String, buffer_num: usize, index: usize) -> Entity{
        Entity{
            buffer_info: EntityBufferInfo{
                model_name: model_name,
                buffer_num: buffer_num,
                index: index,
            },
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