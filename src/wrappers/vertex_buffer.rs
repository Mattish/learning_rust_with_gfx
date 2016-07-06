use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

pub struct VertexBufferWrapper<T : glium::Vertex> {
    remaining: usize,
    last_index: usize,
    pub buffer: VertexBuffer<T>,
    buffer_num: usize,
    total_size: usize,
}

#[derive(Clone,Copy,Debug)]
pub struct VertexBufferStoreInfo {
    pub buffer_num: usize,
    pub start_index: usize,
    pub length: usize,
}

impl<T : glium::Vertex + Sized> VertexBufferWrapper<T> {
    pub fn new(display: &GlutinFacade, size: usize, buffer_num: usize) -> VertexBufferWrapper<T> {
        VertexBufferWrapper {
            remaining: size,
            last_index: 0,
            buffer: glium::VertexBuffer::empty_dynamic(display, size).unwrap(),
            buffer_num: buffer_num,
            total_size: size,
        }
    }

    pub fn clear(&mut self){
        self.remaining = self.total_size;
        self.last_index = 0;
    }

    pub fn add(&mut self, input_array: &[T]) -> Option<VertexBufferStoreInfo> {
        let array_len = input_array.len();
        if array_len > self.remaining {
            return None;
        }
        self.buffer.invalidate();
        let mut counter = self.last_index;
        let buffer_slice = self.buffer.slice_mut(counter..counter+array_len).unwrap();
        let store_info = VertexBufferStoreInfo {
            buffer_num: self.buffer_num,
            start_index: self.last_index,
            length: array_len,
        };
        buffer_slice.write(input_array);
        counter += array_len;
        self.remaining -= array_len;
        self.last_index = counter;
        Some(store_info)
    }
}
