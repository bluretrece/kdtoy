#[derive(Debug, Clone)]
pub struct Circle {
    pub point: [i32; 2],
    pub radius: f32,
}

impl Circle {
    pub fn new(point: [i32; 2], radius: f32) -> Self {
        Circle { point, radius }
    }
}

