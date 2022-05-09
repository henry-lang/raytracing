mod buffer;

use std::fs::File;
use std::io;
use std::sync::{atomic::Ordering, Arc};
use std::thread;
use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use raytracer::{
    camera::{Camera, CameraConfig},
    color::{color, Color},
    hit::Hit,
    image_writer::ImageWriter,
    material::{Material, ScatterResult},
    ray::Ray,
    scene::{Scene, Sky},
    sphere::Sphere,
    vector::vector3,
};

use buffer::AtomicPixel;

fn ray_color(ray: &Ray, scene: &Scene, depth: usize, rand: &mut impl Rng) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(hit_data) = scene.hit(ray, 0.001, f64::INFINITY) {
        match hit_data.material.scatter(ray, &hit_data, rand) {
            ScatterResult::Absorbed => color(0.0, 0.0, 0.0),
            ScatterResult::Scattered {
                attenuation,
                scattered,
            } => attenuation * ray_color(&scattered, &scene, depth - 1, rand),
        }
    } else {
        let normalized = ray.direction.normalize();
        let t = (normalized.y + 1.0) * 0.5;
        scene.sky.get_color(t)
    }
}

fn main() -> io::Result<()> {
    let scene = Arc::new(Scene {
        sky: Sky {
            top: color(0.5, 0.7, 1.0),
            bottom: color(1.0, 1.0, 1.0),
        },
        objects: vec![
            Box::new(Sphere {
                center: vector3(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Lambertian {
                    albedo: color(0.8, 0.8, 0.0),
                },
            }),
            Box::new(Sphere {
                center: vector3(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Lambertian {
                    albedo: color(1.0, 0.3, 0.3),
                },
            }),
            Box::new(Sphere {
                center: vector3(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal {
                    albedo: color(0.3, 0.3, 0.3),
                    fuzz: 0.5,
                },
            }),
            Box::new(Sphere {
                center: vector3(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal {
                    albedo: color(0.8, 0.0, 0.0),
                    fuzz: 0.5,
                },
            }),
        ],
    });

    let aspect_ratio = 16.0 / 9.0;
    let thread_count = num_cpus::get();
    let samples_per_thread = 100;
    let max_depth = 50;

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let buffer = Arc::new(
        (0..image_width * image_height)
            .map(|_| AtomicPixel::new(0.0, 0.0, 0.0))
            .collect::<Vec<_>>(),
    );

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let camera = Arc::new(Camera::new(CameraConfig {
        position: vector3(0.0, 0.0, 0.0),
        viewport_width,
        viewport_height,
        focal_length,
    }));

    println!("Running on {} threads", thread_count);
    let start = Instant::now();

    let handles = (0..thread_count)
        .map(|_| {
            let scene_ref = scene.clone();
            let camera_ref = camera.clone();
            let buffer_ref = buffer.clone();

            thread::spawn(move || {
                let thread_rand = rand::thread_rng();
                let mut rand = XorShiftRng::from_rng(thread_rand).unwrap();

                for _ in 0..samples_per_thread {
                    for j in (0..image_height).rev() {
                        for i in 0..image_width {
                            let u = (i as f64 + rand.gen::<f64>()) / (image_width - 1) as f64;
                            let v = (j as f64 + rand.gen::<f64>()) / (image_height - 1) as f64;
                            let sample = ray_color(
                                &camera_ref.get_ray(u, v),
                                &scene_ref,
                                max_depth,
                                &mut rand,
                            );

                            let index = (image_height - j - 1) * image_width + i;
                            buffer_ref[index].r.store(
                                buffer_ref[index].r.load(Ordering::Relaxed) + sample.r,
                                Ordering::Relaxed,
                            );
                            buffer_ref[index].g.store(
                                buffer_ref[index].g.load(Ordering::Relaxed) + sample.g,
                                Ordering::Relaxed,
                            );
                            buffer_ref[index].b.store(
                                buffer_ref[index].b.load(Ordering::Relaxed) + sample.b,
                                Ordering::Relaxed,
                            );
                        }
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start.elapsed();
    println!("Image rendered in {}ms", elapsed.as_millis());

    let start = Instant::now();

    let mut writer = ImageWriter::new(File::create("image.ppm")?, image_width, image_height);
    for pixel in buffer.as_ref() {
        writer.write_pixel(
            Color {
                r: pixel.r.load(Ordering::Relaxed),
                g: pixel.g.load(Ordering::Relaxed),
                b: pixel.b.load(Ordering::Relaxed),
            },
            samples_per_thread * thread_count,
        )
    }

    let elapsed = start.elapsed();
    println!("Image written in {}ms", elapsed.as_millis());

    Ok(())
}
