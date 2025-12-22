pub struct Vertex {
    pub position: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 4]) -> Self {
        Vertex { position }
    }
}

pub fn parse_vertex(arr: Vec<&str>) -> Result<Vertex, String> {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;
    let mut w: f32 = 0.0;

    if (arr.len() < 4) {
        Error("Error\nNot Enough Arguments for v".to_string())
    } else if (arr.len() >= 4) {
        x = arr[1].parse().unwrap();
        y = arr[2].parse().unwrap();
        z = arr[3].parse().unwrap();
    }

    Ok(Vertex::new([x, y, z, w]))
}
