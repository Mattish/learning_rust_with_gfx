use vertex;
use vertex::Vertex;
use vertex::Normal;

pub struct Teapot{
    pub id: &'static str,
    vertices: &'static[Vertex],
    normals: &'static[Normal],
    indicies: &'static[u16]
}

impl Teapot{
    fn new() -> Teapot{
        let newTeapot = Teapot{
            id: "MyTeapot",
            vertices: &vertex::VERTICES,
            normals:&vertex::NORMALS,
            indicies:&vertex::INDICES
        };
    }
}
