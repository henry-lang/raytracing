use std::fs::File;
use std::io::{BufWriter, Write};

use crate::color::Color;
use crate::Number;

// Simple writer for the ppm image file format. I know it's terrible, but it's dead simple.

pub struct ImageWriter {
    buffer: BufWriter<File>,
}

impl ImageWriter {
    pub fn new(file: File, width: usize, height: usize) -> Self {
        let mut buffer = BufWriter::new(file);
        buffer
            .write(format!("P3 {} {} 255\n", width, height).as_bytes())
            .expect("write image header");

        Self { buffer }
    }

    pub fn write_pixel(&mut self, mut color: Color, samples: usize) {
        color *= 1.0 / samples as Number;

        let r = color.r.sqrt();
        let g = color.g.sqrt();
        let b = color.b.sqrt();

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

        let (s, v) = channel_to_digits((r * 256.0) as u8);
        let r = &v[s..];
        let (s, v) = channel_to_digits((g * 256.0) as u8);
        let g = &v[s..];
        let (s, v) = channel_to_digits((b * 256.0) as u8);
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
