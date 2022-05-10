use atomic_float::AtomicF32;

pub struct AtomicPixel {
    pub r: AtomicF32,
    pub g: AtomicF32,
    pub b: AtomicF32,
}

impl AtomicPixel {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: AtomicF32::new(r),
            g: AtomicF32::new(g),
            b: AtomicF32::new(b),
        }
    }
}
