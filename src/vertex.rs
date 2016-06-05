use vecmath::Vector3;

#[derive(Clone,Copy)]
pub struct Vertex {
    pub vertex: Vector3<f32>
}

#[derive(Clone,Copy)]
pub struct Attr{
    pub attr: Vector3<f32>
}

implement_vertex!(Vertex, vertex);
implement_vertex!(Attr, attr);
