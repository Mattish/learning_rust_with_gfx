use std::collections::HashMap;
use vertex;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Surface;
use glium::Frame;
use glium::uniforms::Uniforms;
use glium::Program;
use glium::DrawParameters;
use wrappers::*;
use entity_model_packer::EntityPackage;

pub struct BufferStore {
    vertex_buffers: Vec<VertexBufferWrapper<vertex::Vertex>>,
    attr_buffers: Vec<VertexBufferWrapper<vertex::Attr>>,
    index_buffers: Vec<IndexBufferWrapper>,
    models: HashMap<String,ModelInfo>,
}

impl BufferStore {
    pub fn new() -> BufferStore {
        BufferStore {
            vertex_buffers: Vec::new(),
            attr_buffers: Vec::new(),
            index_buffers: Vec::new(),
            models: HashMap::new(),
        }
    }

    pub fn draw<U : Uniforms>(&mut self, target: &mut Frame, program: &Program, uniforms: &U, params: &DrawParameters, ent_pack: EntityPackage)
    {
        for key in ent_pack.each.keys(){
            match self.models.get(key){
                Some(model) => {
                    let index_start = model.index_buffer_info.start_index;
                    let index_end = model.index_buffer_info.start_index + model.index_buffer_info.length;

                    match ent_pack.each.get(key){
                        Some(model_instances) => {
                                                    
                            //TODO: This doesn't work for model_instances split over many buffers 
                            
                            target.draw((
                                &self.vertex_buffers[model.model_buffer_info.buffer_num].buffer,

                                self.attr_buffers[0].buffer
                                    .slice(model_instances.start..model_instances.end).unwrap().per_instance().unwrap()),
                                
                                self.index_buffers[model.index_buffer_info.buffer_num].buffer
                                    .slice(index_start..index_end).unwrap(),

                                program,
                                uniforms,
                                params).unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            };
        }
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
        println!("Loaded model:{:?}",model_info);
        model_info
    }

    pub fn input_attr_range(&mut self,display: &GlutinFacade, attr: &[vertex::Attr]) -> VertexBufferStoreInfo {
        for i in 0..self.attr_buffers.len(){
            self.attr_buffers[i].clear();
            match self.attr_buffers[i].add(attr) {
                Some(store_info) => {
                    return store_info;
                    },
                _ => {}
            }
        }
        println!("Creating new attr buffer index:{}",self.attr_buffers.len());
        let mut new_wrapper = VertexBufferWrapper::new(display,10000,self.attr_buffers.len());
        let store_info = new_wrapper.add(attr).unwrap();
        self.attr_buffers.push(new_wrapper);
        store_info
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

#[derive(Clone,Copy,Debug)]
pub struct ModelInfo{
    id: usize,
    model_buffer_info: VertexBufferStoreInfo,
    index_buffer_info: IndexBufferStoreInfo,
}
