use vertex;
use std::io::BufReader;
use std::fs::File;
use obj;

pub fn load_obj_vertices(name: &'static str) -> Vec<vertex::Vertex>{
    let f = File::open(name).unwrap();
    let reader = BufReader::new(f);
    let loaded_obj: obj::Obj = obj::load_obj(reader).unwrap();

    let obj_vertices = &loaded_obj.vertices;
    println!("indicies.len: {}",loaded_obj.indices.len());
    println!("vertices.len: {}",obj_vertices.len());

    let mut verts = Vec::new();
    for index in &mut loaded_obj.indices.into_iter() {
        let vert = obj_vertices[index as usize];
        verts.push(vertex::Vertex{vertex:vert.position})
    }
    
    return verts;
}