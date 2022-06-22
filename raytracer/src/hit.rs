use std::fmt::Display;

use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;
use crate::Number;

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
    pub t: Number,
    pub normal: Vector3,
    pub point: Vector3,
    pub face: Face,
    pub material: &'a Material,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: Number, t_max: Number) -> Option<HitData>;
    fn name(&self) -> &'static str;
}
