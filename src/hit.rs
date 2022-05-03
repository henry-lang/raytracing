use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Copy, Clone)]
pub enum Face {
    Inwards,
    Outwards,
}

impl Face {
    pub fn get(ray: &Ray, normal: &Vector3) -> Self {
        if ray.direction.dot(normal) < 0.0 {
            Self::Outwards
        } else {
            Self::Inwards
        }
    }
}

#[derive(Copy, Clone)]
pub struct HitData<'a> {
    pub t: f64,
    pub normal: Vector3,
    pub point: Vector3,
    pub face: Face,
    pub material: &'a Material,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitData>;
}
