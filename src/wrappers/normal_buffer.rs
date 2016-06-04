use vertex;
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

pub struct NormalBufferWrapper {
    id: usize,
    total: usize,
    remaining: usize,
    last_index: usize,
    pub buffer: VertexBuffer<vertex::Normal>,
}

pub struct NormalBufferStoreInfo {
    pub buffer_num: usize,
    pub start_index: usize,
    pub length: usize,
}

impl NormalBufferWrapper {
    pub fn new(display: &GlutinFacade, size: usize, id: usize) -> NormalBufferWrapper {
        NormalBufferWrapper {
            id: id,
            total: size,
            remaining: size,
            last_index: 0,
            buffer: glium::VertexBuffer::empty_dynamic(display, size).unwrap(),
        }
    }

    pub fn add(&mut self, input_array: &[vertex::Normal]) -> Option<NormalBufferStoreInfo> {
        let array_len = input_array.len();
        if array_len > self.remaining {
            return None;
        }

        self.buffer.invalidate();
        let mut writer_mapping = self.buffer.map_write();
        let mut counter = self.last_index;
        let store_info = NormalBufferStoreInfo {
            buffer_num: 0,
            start_index: self.last_index,
            length: array_len,
        };
        for v in input_array.into_iter() {
            writer_mapping.set(counter, vertex::Normal { normal: v.normal });
            counter = counter + 1;
        }
        self.remaining = self.remaining - array_len;
        self.last_index = counter;
        Some(store_info)
    }
}
