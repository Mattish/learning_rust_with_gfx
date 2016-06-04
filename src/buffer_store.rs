use vertex;
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::IndexBuffer;
use glium::Surface;
use glium::Frame;
use glium::uniforms::Uniforms;
use glium::Program;
use glium::DrawParameters;
use wrappers::*;

pub struct BufferStore {
    pos_buffers: Vec<VertexBufferWrapper>,
    normal_buffers: Vec<NormalBufferWrapper>,
    index_buffers: Vec<IndexBufferWrapper>,
}

impl BufferStore {
    pub fn new() -> BufferStore {
        BufferStore {
            pos_buffers: Vec::new(),
            normal_buffers: Vec::new(),
            index_buffers: Vec::new(),
        }
    }

    pub fn draw<U : Uniforms>(&mut self, target: &mut Frame, program: &Program, uniforms: &U, params: &DrawParameters)
    {
        target.draw((&self.pos_buffers[0].buffer,&self.normal_buffers[0].buffer),&self.index_buffers[0].buffer,program,uniforms,params).unwrap();
    }

    pub fn input_verticies(&mut self,display: &GlutinFacade, input_array: &[vertex::Vertex]) -> VertexBufferStoreInfo {
        for wrapper in self.pos_buffers.iter_mut() {
            match wrapper.add(input_array) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        let mut new_wrapper = VertexBufferWrapper::new(display,8096,self.pos_buffers.len());
        let store_info = new_wrapper.add(input_array).unwrap();
        self.pos_buffers.push(new_wrapper);
        store_info
    }

    pub fn input_normals(&mut self,display: &GlutinFacade, input_array: &[vertex::Normal]) -> NormalBufferStoreInfo {
        for wrapper in self.normal_buffers.iter_mut() {
            match wrapper.add(input_array) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        let mut new_wrapper = NormalBufferWrapper::new(display,8096,self.normal_buffers.len());
        let store_info = new_wrapper.add(input_array).unwrap();
        self.normal_buffers.push(new_wrapper);
        store_info
    }

    pub fn input_indices(&mut self,display: &GlutinFacade, input_array: &[u16]) -> IndexBufferStoreInfo {
        for wrapper in self.index_buffers.iter_mut() {
            match wrapper.add(input_array) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        let mut new_wrapper = IndexBufferWrapper::new(display,8096,self.index_buffers.len());
        let store_info = new_wrapper.add(input_array).unwrap();
        self.index_buffers.push(new_wrapper);
        store_info
    }
}
