use std::fmt::Display;

use crate::color::{color, Color};
use crate::hit::{Hit, HitData};
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::Number;

use rand::Rng;

pub struct Sky {
    pub top: Color,
    pub bottom: Color,
}

impl Sky {
    pub fn get_color(&self, t: Number) -> Color {
        self.bottom * (1.0 - t) + self.top * t
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Hit>>,
    pub sky: Sky,
}

impl Hit for Scene {
    fn hit(&self, ray: &Ray, t_min: Number, t_max: Number) -> Option<HitData> {
        let mut best = None;
        let mut best_distance = t_max;

        for object in &self.objects {
            if let Some(hit_data) = object.hit(ray, t_min, best_distance) {
                best = Some(hit_data);
                best_distance = hit_data.t;
            }
        }

        best
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene")
    }
}

impl Scene {
    pub fn ray_color(&self, ray: &Ray, depth: usize, rand: &mut impl Rng) -> Color {
        if depth <= 0 {
            return color(0.0, 0.0, 0.0);
        }

        if let Some(hit_data) = self.hit(ray, 0.001, f32::INFINITY) {
            match hit_data.material.scatter(ray, &hit_data, rand) {
                ScatterResult::Absorbed => color(0.0, 0.0, 0.0),
                ScatterResult::Scattered {
                    attenuation,
                    scattered,
                } => attenuation * self.ray_color(&scattered, depth - 1, rand),
            }
        } else {
            let normalized = ray.direction.normalize();
            let t = (normalized.y + 1.0) * 0.5;
            self.sky.get_color(t)
        }
    }
}
