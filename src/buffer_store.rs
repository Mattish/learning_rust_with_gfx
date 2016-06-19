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
    instance_pos_buffers: Vec<VertexBufferWrapper<vertex::Attr>>,
    index_buffers: Vec<IndexBufferWrapper>,
    models: HashMap<String,ModelInfo>
}

impl BufferStore {
    pub fn new() -> BufferStore {
        BufferStore {
            pos_buffers: Vec::new(),
            instance_pos_buffers: Vec::new(),
            index_buffers: Vec::new(),
            models: HashMap::new(),
        }
    }

    // pub fn draw<U : Uniforms>(&mut self, target: &mut Frame, program: &Program, uniforms: &U, params: &DrawParameters)
    // {
    //     target.draw((self.pos_buffers[0].buffer.slice(0..36).unwrap(),
    //                  self.instance_pos_buffers[0].buffer.slice(0..100).unwrap().per_instance().unwrap()),
    //                  &self.index_buffers[0].buffer.slice(0..36).unwrap(),
    //                  program,uniforms,params).unwrap();
    // }

    pub fn draw<U : Uniforms>(&mut self, target: &mut Frame, program: &Program, uniforms: &U, params: &DrawParameters, model_name: &str)
    {
        match self.models.get(model_name){
            Some(model) => {
                            let vertex_start = model.model_buffer_info.start_index;
                            let vertex_end = model.model_buffer_info.start_index + model.model_buffer_info.length;
                            let index_start = model.index_buffer_info.start_index;
                            let index_end = model.index_buffer_info.start_index + model.index_buffer_info.length;
                            target.draw((
                                self.pos_buffers[0].buffer.slice(vertex_start..vertex_end).unwrap(),
                                self.instance_pos_buffers[0].buffer.slice(0..100).unwrap().per_instance().unwrap()),
                                &self.index_buffers[0].buffer.slice(index_start..index_end).unwrap(),
                                program,
                                uniforms,
                                params).unwrap();
                     }
            _ => {}
        };
    }

    pub fn load_model(&mut self,display: &GlutinFacade, name: &str, verticies: &[vertex::Vertex], indicies: &[u16]) -> ModelInfo {
        let vertex_info = self.input_verticies(display,verticies);
        let index_info = self.input_indices(display, indicies);
        let model_info = ModelInfo{
            id: self.models.len(),
            model_buffer_info: vertex_info,
            index_buffer_info: index_info,
        };
        self.models.insert(name.to_string(),model_info);
        model_info
    }

    pub fn input_attr_range(&mut self,display: &GlutinFacade, attr: &[vertex::Attr]) -> VertexBufferStoreInfo {
        for wrapper in self.instance_pos_buffers.iter_mut() {
            match wrapper.add(attr) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        let mut new_wrapper = VertexBufferWrapper::new(display,100);
        let store_info = new_wrapper.add(attr).unwrap();
        self.instance_pos_buffers.push(new_wrapper);
        store_info
    }

    pub fn update_attr(&mut self, index: usize, value: vertex::Attr){
        self.instance_pos_buffers[0].update(index,value);
    }

    fn input_verticies(&mut self,display: &GlutinFacade, input_array: &[vertex::Vertex]) -> VertexBufferStoreInfo {
        for wrapper in self.pos_buffers.iter_mut() {
            match wrapper.add(input_array) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        let mut new_wrapper = VertexBufferWrapper::new(display,47112);
        let store_info = new_wrapper.add(input_array).unwrap();
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

        let mut new_wrapper = IndexBufferWrapper::new(display,47112);
        let store_info = new_wrapper.add(input_array).unwrap();
        self.index_buffers.push(new_wrapper);
        store_info
    }
}

#[derive(Clone,Copy)]
pub struct ModelInfo{
    id: usize,
    model_buffer_info: VertexBufferStoreInfo,
    index_buffer_info: IndexBufferStoreInfo,
}
