use vertex::Vertex;

pub const VERTICES: [Vertex; 8] = [
    // front
    Vertex { vertex: [-1.0, -1.0,  1.0] },
    Vertex { vertex: [1.0, -1.0,  1.0] },
    Vertex { vertex: [1.0,  1.0,  1.0] },
    Vertex { vertex: [-1.0,  1.0,  1.0] },
    // back
    Vertex { vertex: [-1.0, -1.0, -1.0] },
    Vertex { vertex: [1.0, -1.0, -1.0] },
    Vertex { vertex: [1.0,  1.0, -1.0] },
    Vertex { vertex: [-1.0,  1.0, -1.0] },
];

//pub const NORMALS: [Vertex; 531] = [
//    Vertex { vertex: [0.0, 0.0, 0.0] },     // dummy vector because in the original model indices
//]

pub const INDICES: [u16; 36] = [
// front
    0, 1, 2,
    2, 3, 0,
    // top
    1, 5, 6,
    6, 2, 1,
    // back
    7, 6, 5,
    5, 4, 7,
    // bottom
    4, 0, 3,
    3, 7, 4,
    // left
    4, 5, 1,
    1, 0, 4,
    // right
    3, 2, 6,
    6, 7, 3,
];
