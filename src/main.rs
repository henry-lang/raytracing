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

use color::{color, Color};
use hit::Hit;
use image_writer::ImageWriter;
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vector::vector3;

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    if let Some(hit_data) = scene.hit(ray, 0.0, f64::INFINITY) {
        let normal_color: Color = hit_data.normal.into();
        (normal_color + color(1.0, 1.0, 1.0)) / 2.0
    } else {
        let normalized = ray.direction.normalize();
        let t = (normalized.y + 1.0) / 2.0;
        color(1.0, 1.0, 1.0) * (1.0 - t) + color(0.5, 0.7, 1.0) * t
    }
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                center: vector3(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: vector3(3.0, 0.0, -3.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: vector3(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = vector3(0.0, 0.0, 0.0);
    let horizontal = vector3(viewport_width, 0.0, 0.0);
    let vertical = vector3(0.0, viewport_height, 0.0);
    let bottom_left = origin - horizontal / 2.0 - vertical / 2.0 - vector3(0.0, 0.0, focal_length);

    let start = Instant::now();
    let mut writer = ImageWriter::new(File::create("image.ppm")?, image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;

            let ray = Ray {
                origin,
                direction: bottom_left + horizontal * u + vertical * v - origin,
            };
            writer.write_pixel(ray_color(&ray, &scene));
        }
    }

    writer.flush();

    let elapsed = start.elapsed();
    println!("{}ms", elapsed.as_millis());

    Ok(())
}
