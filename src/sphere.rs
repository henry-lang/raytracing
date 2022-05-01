use crate::hit::Face;
use crate::hit::{Hit, HitData};
use crate::ray::Ray;
use crate::vector::Vector3;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let distance = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = ray.direction.dot(&distance);
        let c = distance.length_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-b - sqrt_d) / a;
        if t < t_min || t > t_max {
            t = (-b + sqrt_d) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.at(t);
        let mut normal = (point - self.center) / self.radius;
        let face = Face::get(ray, &normal);
        normal = match face {
            Face::Outwards => normal,
            Face::Inwards => -normal,
        };

        Some(HitData {
            t,
            point,
            normal,
            face,
        })
    }
}
