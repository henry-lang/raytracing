mod color;
mod image_writer;

use color::Color;
use image_writer::ImageWriter;
use std::fs::File;
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let width = 256;
    let height = 256;

    let start = Instant::now();
    let mut writer = ImageWriter::new(File::create("image.ppm")?, width, height);
    for i in 0..width {
        for j in 0..height {
            writer.write_pixel(Color {
                r: (i as f64) / width as f64,
                g: (j as f64) / height as f64,
                b: 0.25,
            });
        }
    }

    writer.flush();

    let elapsed = start.elapsed();
    println!("{}ms", elapsed.as_millis());

    Ok(())
}
