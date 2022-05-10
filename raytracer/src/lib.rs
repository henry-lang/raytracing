pub mod camera;
pub mod color;
pub mod hit;
pub mod image_writer;
pub mod material;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod vector;

#[cfg(all(feature = "f32", not(feature = "f64")))]
pub type Number = f32;

#[cfg(all(feature = "f64", not(feature = "f32")))]
pub type Number = f64;
