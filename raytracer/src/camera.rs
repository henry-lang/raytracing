use crate::ray::Ray;
use crate::vector::{vector3, Vector3};

pub struct CameraConfig {
    pub position: Vector3,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
}

pub struct Camera {
    position: Vector3,
    bottom_left: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        let horizontal = vector3(config.viewport_width, 0.0, 0.0);
        let vertical = vector3(0.0, config.viewport_height, 0.0);
        let bottom_left = config.position
            - horizontal / 2.0
            - vertical / 2.0
            - vector3(0.0, 0.0, config.focal_length);

        Self {
            position: config.position,
            bottom_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.position,
            direction: self.bottom_left + self.horizontal * u + self.vertical * v - self.position,
        }
    }
}
