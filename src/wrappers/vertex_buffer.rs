use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::VertexBuffer;

pub struct VertexBufferWrapper<T : glium::Vertex> {
    remaining: usize,
    last_index: usize,
    pub buffer: VertexBuffer<T>,
    buffer_num: usize,
}

#[derive(Clone,Copy,Debug)]
pub struct VertexBufferStoreInfo {
    pub buffer_num: usize,
    pub start_index: usize,
    pub length: usize,
}

pub struct AttrUpdate<T : glium::Vertex>{
    pub index: usize,
    pub buffer_num: usize,
    pub attr: T,
}

impl<T : glium::Vertex + Sized> VertexBufferWrapper<T> {
    pub fn new(display: &GlutinFacade, size: usize, buffer_num: usize) -> VertexBufferWrapper<T> {
        VertexBufferWrapper {
            remaining: size,
            last_index: 0,
            buffer: glium::VertexBuffer::empty_dynamic(display, size).unwrap(),
            buffer_num: buffer_num
        }
    }

    pub fn add(&mut self, input_array: &[T]) -> Option<VertexBufferStoreInfo> {
        let array_len = input_array.len();
        if array_len > self.remaining {
            return None;
        }

        let mut counter = self.last_index;
        let buffer_slice = self.buffer.slice_mut(counter..counter+array_len).unwrap();
        let store_info = VertexBufferStoreInfo {
            buffer_num: self.buffer_num,
            start_index: self.last_index,
            length: array_len,
        };
        buffer_slice.write(input_array);
        counter = counter + array_len;
        self.remaining = self.remaining - array_len;
        self.last_index = counter;
        Some(store_info)
    }

    pub fn update_many(&mut self, inputs: &[AttrUpdate<T>]){
        let mut buffer_slice = self.buffer.map_write();
        for i in 0..inputs.len(){
            buffer_slice.set(inputs[i].index, inputs[i].attr);
        }
    }


}
