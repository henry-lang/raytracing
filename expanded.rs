#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod color {
    pub struct Color {
        pub r: f64,
        pub g: f64,
        pub b: f64,
    }
}
mod image_writer {
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use crate::color::Color;
    pub struct ImageWriter {
        buffer: BufWriter<File>,
    }
    impl ImageWriter {
        pub fn new(file: File, width: usize, height: usize) -> Self {
            let mut buffer = BufWriter::new(file);
            buffer
                .write(
                    {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["P3 ", " ", " 255\n"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&width),
                                ::core::fmt::ArgumentV1::new_display(&height),
                            ],
                        ));
                        res
                    }
                    .as_bytes(),
                )
                .expect("write image header");
            Self { buffer }
        }
        pub fn write_pixel(&mut self, color: Color) {
            let channel_to_digits = |mut c: u8| -> (usize, [u8; 3]) {
                let s = 0;
                let mut v = [0, 0, 0];
                for i in 0..3 {
                    let digit = b'0' + (c % 10);
                    v[2 - i] = digit;
                    c /= 10;
                }
                (s, v)
            };
            let (s, v) = channel_to_digits((color.r * 256.) as u8);
            let r = &v[s..];
            let (s, v) = channel_to_digits((color.g * 256.) as u8);
            let g = &v[s..];
            let (s, v) = channel_to_digits((color.b * 256.) as u8);
            let b = &v[s..];
            self.buffer.write(r).expect("write pixel");
            self.buffer.write(b" ").expect("write pixel");
            self.buffer.write(g).expect("write pixel");
            self.buffer.write(b" ").expect("write pixel");
            self.buffer.write(b).expect("write pixel");
            self.buffer.write(b"\n").expect("write pixel");
        }
        pub fn flush(&mut self) {
            self.buffer.flush().unwrap();
        }
    }
}
mod ray {
    use crate::vector::Vector3;
    struct Ray {
        pub origin: Vector3,
        pub direction: Vector3,
    }
    impl Ray {
        pub fn at(&self, t: f64) -> Vector3 {
            self.origin + self.direction * t
        }
    }
}
mod vector {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Vector3 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Vector3 {
                    x: ref __self_0_0,
                    y: ref __self_0_1,
                    z: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Vector3");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "x", &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "y", &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "z", &&(*__self_0_2));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Vector3 {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Vector3 {
        #[inline]
        fn clone(&self) -> Vector3 {
            {
                let _: ::core::clone::AssertParamIsClone<f64>;
                let _: ::core::clone::AssertParamIsClone<f64>;
                let _: ::core::clone::AssertParamIsClone<f64>;
                *self
            }
        }
    }
    impl Vector3 {
        pub fn length(&self) -> f64 {
            f64::sqrt(self.dot(self))
        }
        pub fn dot(&self, other: &Self) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
        pub fn cross(&self, other: &Self) -> Self {
            Vector3 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }
        pub fn normalize(&self) -> Self {
            *self / self.length()
        }
    }
    impl Neg for Vector3 {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }
    impl Add for Vector3 {
        type Output = Self;
        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl Add<f64> for Vector3 {
        type Output = Self;
        fn add(self, other: f64) -> Self::Output {
            Self {
                x: self.x + other,
                y: self.y + other,
                z: self.z + other,
            }
        }
    }
    impl AddAssign for Vector3 {
        fn add_assign(&mut self, other: Self) {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
        }
    }
    impl AddAssign<f64> for Vector3 {
        fn add_assign(&mut self, other: f64) {
            self.x += other;
            self.y += other;
            self.z += other;
        }
    }
    impl Sub for Vector3 {
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }
    impl Sub<f64> for Vector3 {
        type Output = Self;
        fn sub(self, other: f64) -> Self::Output {
            Self {
                x: self.x - other,
                y: self.y - other,
                z: self.z - other,
            }
        }
    }
    impl SubAssign for Vector3 {
        fn sub_assign(&mut self, other: Self) {
            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
        }
    }
    impl SubAssign<f64> for Vector3 {
        fn sub_assign(&mut self, other: f64) {
            self.x -= other;
            self.y -= other;
            self.z -= other;
        }
    }
    impl Mul for Vector3 {
        type Output = Self;
        fn mul(self, other: Self) -> Self::Output {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
    impl Mul<f64> for Vector3 {
        type Output = Self;
        fn mul(self, other: f64) -> Self::Output {
            Self {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }
    impl MulAssign for Vector3 {
        fn mul_assign(&mut self, other: Self) {
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
        }
    }
    impl MulAssign<f64> for Vector3 {
        fn mul_assign(&mut self, other: f64) {
            self.x *= other;
            self.y *= other;
            self.z *= other;
        }
    }
    impl Div for Vector3 {
        type Output = Self;
        fn div(self, other: Self) -> Self::Output {
            Self {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            }
        }
    }
    impl Div<f64> for Vector3 {
        type Output = Self;
        fn div(self, other: f64) -> Self::Output {
            Self {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
            }
        }
    }
    impl DivAssign for Vector3 {
        fn div_assign(&mut self, other: Self) {
            self.x /= other.x;
            self.y /= other.y;
            self.z /= other.z;
        }
    }
    impl DivAssign<f64> for Vector3 {
        fn div_assign(&mut self, other: f64) {
            self.x /= other;
            self.y /= other;
            self.z /= other;
        }
    }
}
use std::fs::File;
use std::io;
use std::time::Instant;
use crate::vector::Vector3;
use color::Color;
use image_writer::ImageWriter;
fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;
    let viewport_width = 2.0;
    let viewport_height = viewport_width / aspect_ratio;
    let focal_length = 1.0;
    let origin = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let start = Instant::now();
    let mut writer = ImageWriter::new(File::create("image.ppm")?, width, height);
    for i in 0..height {
        for j in 0..width {
            writer.write_pixel(Color {
                r: (i as f64) / width as f64,
                g: (j as f64) / height as f64,
                b: 0.25,
            });
        }
    }
    writer.flush();
    let elapsed = start.elapsed();
    ::std::io::_print(::core::fmt::Arguments::new_v1(
        &["", "ms\n"],
        &[::core::fmt::ArgumentV1::new_display(&elapsed.as_millis())],
    ));
    Ok(())
}
