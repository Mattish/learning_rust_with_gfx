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
    vertex_buffers: Vec<VertexBufferWrapper<vertex::Vertex>>,
    attr_buffers: Vec<VertexBufferWrapper<vertex::Attr>>,
    attr_updates: Vec<Vec<AttrUpdate<vertex::Attr>>>,
    index_buffers: Vec<IndexBufferWrapper>,
    models: HashMap<String,ModelInfo>,
    model_instances: HashMap<String,Vec<VertexBufferStoreInfo>>
}

impl BufferStore {
    pub fn new() -> BufferStore {
        BufferStore {
            vertex_buffers: Vec::new(),
            attr_buffers: Vec::new(),
            index_buffers: Vec::new(),
            attr_updates: Vec::new(),
            models: HashMap::new(),
            model_instances: HashMap::new(),
        }
    }

    pub fn draw<U : Uniforms>(&mut self, target: &mut Frame, program: &Program, uniforms: &U, params: &DrawParameters, model_name: &str)
    {
        match self.models.get(model_name){
            Some(model) => {
                let vertex_start = model.model_buffer_info.start_index;
                let vertex_end = model.model_buffer_info.start_index + model.model_buffer_info.length;
                let index_start = model.index_buffer_info.start_index;
                let index_end = model.index_buffer_info.start_index + model.index_buffer_info.length;

                let model_instances = self.model_instances.get(model_name).unwrap();
                let first_model_instance = model_instances[0];
                let last_model_instance = model_instances[model_instances.len() - 1];

                let model_instances_start = first_model_instance.start_index;
                let model_instances_end = last_model_instance.start_index;
                
                //TODO: This doesn't work for model_instances split over many buffers 
                target.draw((
                    self.vertex_buffers[model.model_buffer_info.buffer_num].buffer.
                        slice(vertex_start..vertex_end).unwrap(),
                    self.attr_buffers[first_model_instance.buffer_num].buffer.
                        slice(model_instances_start..model_instances_end).unwrap().per_instance().unwrap()),
                    &self.index_buffers[model.index_buffer_info.buffer_num].buffer.
                        slice(index_start..index_end).unwrap(),
                    program,
                    uniforms,
                    params).unwrap();
                
            }
            _ => {}
        };
    }

    pub fn load_model(&mut self,display: &GlutinFacade, name: &str, 
                        verticies: &[vertex::Vertex],
                        indicies: &[u16]) 
                        -> ModelInfo {
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

    pub fn input_attr_range(&mut self,display: &GlutinFacade, attr: &[vertex::Attr], model_name: &str) -> VertexBufferStoreInfo {
        for i in 0..self.attr_buffers.len(){
            match self.attr_buffers[i].add(attr) {
                Some(store_info) => {
                    self.store_model_instance(model_name,store_info);
                    return store_info;
                    },
                _ => {}
            }
        }
        println!("Creating new attr buffer index:{}",self.attr_buffers.len());
        let mut new_wrapper = VertexBufferWrapper::new(display,10000,self.attr_buffers.len());
        let store_info = new_wrapper.add(attr).unwrap();
        self.attr_buffers.push(new_wrapper);
        self.store_model_instance(model_name,store_info);
        self.attr_updates.push(Vec::new());
        store_info
    }

    pub fn update_attr(&mut self, index: usize, buffer_num: usize, value: vertex::Attr){
        self.attr_updates[buffer_num].push(AttrUpdate{
            index: index,
            buffer_num: buffer_num,
            attr: value,
        });
        
    }

    pub fn do_attr_updates(&mut self){
        for i in 0..self.attr_buffers.len(){
            self.attr_buffers[i].update_many(&self.attr_updates[i]);
            self.attr_updates[i].clear();
        }
        
    }

    fn store_model_instance(&mut self, model_name: &str, buffer_store_info: VertexBufferStoreInfo){
        fn new_vec() -> Vec<VertexBufferStoreInfo>{
            Vec::new()
        }
        let mut model_instances = self.model_instances.entry(model_name.to_string()).or_insert_with(new_vec);
        model_instances.push(buffer_store_info);
    }

    fn input_verticies(&mut self,display: &GlutinFacade, input_array: &[vertex::Vertex]) -> VertexBufferStoreInfo {
        for wrapper in self.vertex_buffers.iter_mut() {
            match wrapper.add(input_array) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }
        println!("Creating new vert buffer index:{}",self.vertex_buffers.len());
        let mut new_wrapper = VertexBufferWrapper::new(display,100000,self.vertex_buffers.len());
        let store_info = new_wrapper.add(input_array).unwrap();
        self.vertex_buffers.push(new_wrapper);
        store_info
    }

    fn input_indices(&mut self,display: &GlutinFacade, input_array: &[u16]) -> IndexBufferStoreInfo {
        for wrapper in self.index_buffers.iter_mut() {
            match wrapper.add(input_array) {
                Some(store_info) => return store_info,
                _ => {}
            }
        }

        println!("Creating new indices buffer index:{}",self.index_buffers.len());
        let mut new_wrapper = IndexBufferWrapper::new(display,100000,self.index_buffers.len());
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
