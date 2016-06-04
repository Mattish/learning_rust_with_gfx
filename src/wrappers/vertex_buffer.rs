use vertex;
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

pub struct VertexBufferWrapper {
    id: usize,
    total: usize,
    remaining: usize,
    last_index: usize,
    pub buffer: VertexBuffer<vertex::Vertex>,
}

pub struct VertexBufferStoreInfo {
    pub buffer_num: usize,
    pub start_index: usize,
    pub length: usize,
}

impl VertexBufferWrapper {
    pub fn new(display: &GlutinFacade, size: usize, id: usize) -> VertexBufferWrapper {
        VertexBufferWrapper {
            id: id,
            total: size,
            remaining: size,
            last_index: 0,
            buffer: glium::VertexBuffer::empty_dynamic(display, size).unwrap(),
        }
    }

    pub fn add(&mut self, input_array: &[vertex::Vertex]) -> Option<VertexBufferStoreInfo> {
        let array_len = input_array.len();
        if array_len > self.remaining {
            return None;
        }

        self.buffer.invalidate();
        let mut writer_mapping = self.buffer.map_write();
        let mut counter = self.last_index;
        let store_info = VertexBufferStoreInfo {
            buffer_num: 0,
            start_index: self.last_index,
            length: array_len,
        };
        for v in input_array.into_iter() {
            writer_mapping.set(counter, vertex::Vertex { position: v.position });
            counter = counter + 1;
        }
        self.remaining = self.remaining - array_len;
        self.last_index = counter;
        Some(store_info)
    }
}
