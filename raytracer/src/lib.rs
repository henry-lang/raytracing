mod camera;
mod color;
mod hit;
mod image_writer;
mod material;
mod ray;
mod scene;
mod sphere;
mod vector;

pub use camera::*;
pub use color::*;
pub use hit::*;
pub use image_writer::*;
pub use material::*;
pub use ray::*;
pub use scene::*;
pub use sphere::*;
pub use vector::*;

#[cfg(all(feature = "f32", not(feature = "f64")))]
pub type Number = f32;

#[cfg(all(feature = "f64", not(feature = "f32")))]
pub type Number = f64;
