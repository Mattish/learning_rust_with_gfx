use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

pub struct VertexBufferWrapper<T : glium::Vertex> {
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

impl<T : glium::Vertex + Sized> VertexBufferWrapper<T> {
    pub fn new(display: &GlutinFacade, size: usize) -> VertexBufferWrapper<T> {
        VertexBufferWrapper {
            remaining: size,
            last_index: 0,
            buffer: glium::VertexBuffer::empty_dynamic(display, size).unwrap(),
        }
    }

    pub fn add(&mut self, input_array: &[T]) -> Option<VertexBufferStoreInfo> {
        let array_len = input_array.len();
        if array_len > self.remaining {
            return None;
        }

        //self.buffer.invalidate();

        let mut counter = self.last_index;
        let buffer_slice = self.buffer.as_mut_slice().slice(counter..counter+array_len).unwrap();
        let store_info = VertexBufferStoreInfo {
            buffer_num: 0,
            start_index: self.last_index,
            length: array_len,
        };
        buffer_slice.write(input_array);
        counter = counter + array_len;
        self.remaining = self.remaining - array_len;
        self.last_index = counter;
        Some(store_info)
    }
}
