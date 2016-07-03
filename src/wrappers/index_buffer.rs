use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::IndexBuffer;

pub struct IndexBufferWrapper {
    remaining: usize, 
    last_index: usize,
    pub buffer: IndexBuffer<u16>,
    buffer_num: usize,
}

#[derive(Clone,Copy,Debug)]
pub struct IndexBufferStoreInfo {
    pub buffer_num: usize,
    pub start_index: usize,
    pub length: usize,
}

impl IndexBufferWrapper {
    pub fn new(display: &GlutinFacade, size: usize, buffer_num: usize) -> IndexBufferWrapper {
        IndexBufferWrapper {
            remaining: size,
            last_index: 0,
            buffer: glium::IndexBuffer::empty_dynamic(display, glium::index::PrimitiveType::TrianglesList, size).unwrap(),
            buffer_num: buffer_num
        }
    }

    pub fn add(&mut self, input_array: &[u16]) -> Option<IndexBufferStoreInfo> {
        let array_len = input_array.len();
        if array_len > self.remaining {
            return None;
        }

        self.buffer.invalidate();
        let mut writer_mapping = self.buffer.map_write();
        let mut counter = self.last_index;
        let store_info = IndexBufferStoreInfo {
            buffer_num: self.buffer_num,
            start_index: self.last_index,
            length: array_len,
        };
        for v in input_array.into_iter() {
            writer_mapping.set(counter, *v);
            counter = counter + 1;
        }
        self.remaining = self.remaining - array_len;
        self.last_index = counter;
        Some(store_info)
    }
}
