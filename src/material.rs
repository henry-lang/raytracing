use rand::Rng;

use crate::color::Color;
use crate::hit::HitData;
use crate::ray::Ray;
use crate::vector::Vector3;

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

pub enum ScatterResult {
    Absorbed,
    Scattered { attenuation: Color, scattered: Ray },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_data: &HitData, rand: &mut impl Rng) -> ScatterResult {
        match self {
            Material::Lambertian { albedo } => {
                let mut direction = hit_data.normal + Vector3::random_normalized(rand);

                if direction.near_zero() {
                    direction = hit_data.normal;
                }

                ScatterResult::Scattered {
                    attenuation: *albedo,
                    scattered: Ray {
                        origin: hit_data.point,
                        direction,
                    },
                }
            }

            Material::Metal { albedo, fuzz } => {
                let direction = ray.direction.normalize().reflect(&hit_data.normal)
                    + Vector3::random_in_unit_sphere(rand) * *fuzz;
                if direction.dot(&hit_data.normal) > 0.0 {
                    ScatterResult::Scattered {
                        attenuation: *albedo,
                        scattered: Ray {
                            origin: hit_data.point,
                            direction,
                        },
                    }
                } else {
                    ScatterResult::Absorbed
                }
            }
        }
    }
}
