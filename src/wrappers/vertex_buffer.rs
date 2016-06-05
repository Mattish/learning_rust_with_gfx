use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

pub struct VertexBufferWrapper<T : glium::Vertex> {
    id: usize,
    total: usize,
    remaining: usize,
    last_index: usize,
    pub buffer: VertexBuffer<T>,
}

#[derive(Clone,Copy)]
pub struct VertexBufferStoreInfo {
    pub buffer_num: usize,
    pub start_index: usize,
    pub length: usize,
}

impl<T : glium::Vertex> VertexBufferWrapper<T> {
    pub fn new(display: &GlutinFacade, size: usize, id: usize) -> VertexBufferWrapper<T> {
        VertexBufferWrapper {
            id: id,
            total: size,
            remaining: size,
            last_index: 0,
            buffer: glium::VertexBuffer::empty_dynamic(display, size).unwrap(),
        }
    }

    pub fn add(&mut self, input: &T) -> Option<VertexBufferStoreInfo> {
        if self.remaining <= 0 {
            return None;
        }

        self.buffer.invalidate();
        let mut writer_mapping = self.buffer.map_write();
        let mut counter = self.last_index;
        let store_info = VertexBufferStoreInfo {
            buffer_num: 0,
            start_index: self.last_index,
            length: 1,
        };
        writer_mapping.set(counter, input.clone());
        counter = counter + 1;

        self.remaining = self.remaining - 1;
        self.last_index = counter;
        Some(store_info)
    }

    pub fn add_range(&mut self, input_array: &[T]) -> Option<VertexBufferStoreInfo> {
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
            writer_mapping.set(counter, v.clone());
            counter = counter + 1;
        }
        self.remaining = self.remaining - array_len;
        self.last_index = counter;
        Some(store_info)
    }
}
