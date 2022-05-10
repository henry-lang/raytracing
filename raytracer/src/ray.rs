use crate::vector::Vector3;
use crate::Number;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: Number) -> Vector3 {
        self.origin + self.direction * t
    }
}
