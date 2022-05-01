mod color;
mod image_writer;
mod ray;
mod vector;

use std::fs::File;
use std::io;
use std::time::Instant;

use color::{color, Color};
use image_writer::ImageWriter;
use ray::Ray;
use vector::{vector3, Vector3};

fn hit_sphere(ray: &Ray, center: &Vector3, radius: f64) -> bool {
    let distance = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = ray.direction.dot(&distance) * 2.0;
    let c = distance.dot(&distance) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(ray, &vector3(0.0, 0.0, -1.0), 0.5) {
        return color(1.0, 0.0, 0.0);
    }

    let normalized = ray.direction.normalize();
    let t = 0.5 * (normalized.y + 1.0);

    color(1.0, 1.0, 1.0) * (1.0 - t) + color(0.5, 0.7, 1.0) * t
}

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
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
            writer.write_pixel(ray_color(&ray));
        }
    }

    writer.flush();

    let elapsed = start.elapsed();
    println!("{}ms", elapsed.as_millis());

    Ok(())
}
