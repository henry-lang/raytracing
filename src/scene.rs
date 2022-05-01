use crate::hit::{Hit, HitData};
use crate::ray::Ray;

pub struct Scene {
    pub objects: Vec<Box<dyn Hit>>,
}

impl Hit for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let mut best = Option::<HitData>::None;
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
