#[derive(Copy, Clone)]
pub struct Vect3 {
    pub position: [f32; 3],
}

impl Vect3 {
    pub fn new(position: [f32; 3]) -> Self {
        Vect3 { position }
    }
}
