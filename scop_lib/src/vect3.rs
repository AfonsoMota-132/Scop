#[derive(Copy, Clone)]
pub struct Vect3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vect3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vect3 { x, y, z }
    }
}
