use vecmath::Vector3;

#[derive(Clone,Copy)]
pub struct Vertex {
    pub vertex: Vector3<f32>,
    pub normal: Vector3<f32>,
}

#[derive(Clone,Copy,Debug)]
pub struct Attr {
    pub attr: Vector3<f32>,
    pub scale: f32,
    pub colour: Vector3<f32>,
}

implement_vertex!(Vertex, vertex, normal);
implement_vertex!(Attr, attr, scale, colour);
