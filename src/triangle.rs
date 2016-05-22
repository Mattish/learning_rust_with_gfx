use vertex::Vertex;

pub struct Triangle{
    pub shape: Vec<Vertex>
}

impl Triangle{
    pub fn new() -> Triangle{
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [0.0, 0.5] };
        let vertex3 = Vertex { position: [0.5, -0.25] };
        Triangle{shape: vec![vertex1, vertex2, vertex3]}
    }
}
