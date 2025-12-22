pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Self {
        Vertex { position }
    }
}
