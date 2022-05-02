mod camera;
mod color;
mod hit;
mod image_writer;
mod ray;
mod scene;
mod sphere;
mod vector;

use std::fs::File;
use std::io;
use std::time::Instant;

use rand::{Rng, RngCore};

use camera::{Camera, CameraConfig};
use color::{color, Color};
use hit::Hit;
use image_writer::ImageWriter;
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vector::{vector3, Vector3};

fn ray_color(ray: &Ray, scene: &Scene, depth: usize, rand: &mut impl RngCore) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(hit_data) = scene.hit(ray, 0.001, f64::INFINITY) {
        let target = hit_data.normal + Vector3::random_in_unit_sphere(rand);
        ray_color(
            &Ray {
                origin: hit_data.point,
                direction: target,
            },
            scene,
            depth - 1,
            rand,
        ) * 0.5
    } else {
        let normalized = ray.direction.normalize();
        let t = (normalized.y + 1.0) * 0.5;
        color(1.0, 1.0, 1.0) * (1.0 - t) + color(0.5, 0.7, 1.0) * t
    }
}

fn main() -> io::Result<()> {
    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                center: vector3(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: vector3(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

    let mut rand = rand::thread_rng();

    let aspect_ratio = 1.0;
    let samples = 100;
    let max_depth = 100;

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let camera = Camera::new(CameraConfig {
        position: vector3(0.0, 0.0, 0.0),
        viewport_width,
        viewport_height,
        focal_length,
    });

    let start = Instant::now();
    let mut writer = ImageWriter::new(File::create("image.ppm")?, image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            writer.write_pixel(
                (0..samples)
                    .map(|_| {
                        let u = (i as f64 + rand.gen::<f64>()) / (image_width - 1) as f64;
                        let v = (j as f64 + rand.gen::<f64>()) / (image_height - 1) as f64;
                        ray_color(&camera.get_ray(u, v), &scene, max_depth, &mut rand)
                    })
                    .reduce(|accum, sample| accum + sample)
                    .unwrap(),
                samples,
            );
        }
    }

    writer.flush();

    let elapsed = start.elapsed();
    println!("{}ms", elapsed.as_millis());

    Ok(())
}
