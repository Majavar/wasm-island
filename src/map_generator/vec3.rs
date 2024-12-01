#[derive(Debug, Clone)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }
}
