use std::collections::HashMap;
use vertex;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use glium::Frame;
use glium::uniforms::Uniforms;
use glium::Program;
use glium::DrawParameters;
use wrappers::*;

pub struct BufferStore {
    pos_buffers: Vec<VertexBufferWrapper<vertex::Vertex>>,
    attr_buffers: Vec<VertexBufferWrapper<vertex::Attr>>,
    index_buffers: Vec<IndexBufferWrapper>,
    models: HashMap<usize,ModelInfo>
}

impl BufferStore {
    pub fn new() -> BufferStore {
        BufferStore {
            pos_buffers: Vec::new(),
            attr_buffers: Vec::new(),
            index_buffers: Vec::new(),
            models: HashMap::new(),
        }
    }

    pub fn draw<U : Uniforms>(&mut self, target: &mut Frame, program: &Program, uniforms: &U, params: &DrawParameters)
    {
        target.draw((&self.pos_buffers[0].buffer,self.attr_buffers[0].buffer.per_instance().unwrap()),&self.index_buffers[0].buffer,program,uniforms,params).unwrap();
    }

    pub fn load_model(&mut self,display: &GlutinFacade, verticies: &[vertex::Vertex], indicies: &[u16]) -> ModelInfo {
        let vertex_info = self.input_verticies(display,verticies);
        let index_info = self.input_indices(display, indicies);
        let model_info = ModelInfo{
            id: self.models.len(),
            model_buffer_info: vertex_info,
            index_buffer_indo: index_info,
        };
        self.models.insert(model_info.id,model_info);
        model_info
    }

    pub fn input_attr(&mut self,display: &GlutinFacade, attr: &vertex::Attr) -> VertexBufferStoreInfo {
        for wrapper in self.attr_buffers.iter_mut() {
            match wrapper.add(&attr) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        let mut new_wrapper = VertexBufferWrapper::new(display,8096,self.attr_buffers.len());
        let store_info = new_wrapper.add(attr).unwrap();
        self.attr_buffers.push(new_wrapper);
        store_info
    }

    fn input_verticies(&mut self,display: &GlutinFacade, input_array: &[vertex::Vertex]) -> VertexBufferStoreInfo {
        for wrapper in self.pos_buffers.iter_mut() {
            match wrapper.add_range(input_array) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        let mut new_wrapper = VertexBufferWrapper::new(display,8096,self.pos_buffers.len());
        let store_info = new_wrapper.add_range(input_array).unwrap();
        self.pos_buffers.push(new_wrapper);
        store_info
    }

    fn input_indices(&mut self,display: &GlutinFacade, input_array: &[u16]) -> IndexBufferStoreInfo {
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

#[derive(Clone,Copy)]
pub struct ModelInfo{
    id: usize,
    model_buffer_info: VertexBufferStoreInfo,
    index_buffer_indo: IndexBufferStoreInfo,
}
