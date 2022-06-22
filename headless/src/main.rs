use std::fs::File;
use std::io;
use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use raytracer::{
    color, vector3, Camera, CameraConfig, ImageWriter, Material, Number, Scene, Sky, Sphere,
};

fn main() -> io::Result<()> {
    let scene = Scene {
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
    };

    let aspect_ratio = 16.0 / 9.0;
    let samples = 500;
    let max_depth = 5;

    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let camera = Camera::new(CameraConfig {
        position: vector3(0.0, 0.0, 0.0),
        viewport_width,
        viewport_height,
        focal_length,
    });

    let mut writer = ImageWriter::new(File::create("image.ppm")?, image_width, image_height);

    let start = Instant::now();

    let thread_rand = rand::thread_rng();
    let mut rand = XorShiftRng::from_rng(thread_rand).unwrap();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            writer.write_pixel(
                (0..samples)
                    .map(|_| {
                        let u = (i as Number + rand.gen::<Number>()) / (image_width - 1) as Number;
                        let v = (j as Number + rand.gen::<Number>()) / (image_height - 1) as Number;
                        scene.ray_color(&camera.get_ray(u, v), max_depth, &mut rand)
                    })
                    .reduce(|accum, sample| accum + sample)
                    .unwrap(),
                samples,
            );
        }
    }

    println!("Image rendered in {}ms", start.elapsed().as_millis());

    writer.flush();

    Ok(())
}
