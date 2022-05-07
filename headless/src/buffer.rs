use atomic_float::AtomicF64;

pub struct AtomicPixel {
    pub r: AtomicF64,
    pub g: AtomicF64,
    pub b: AtomicF64,
}

impl AtomicPixel {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: AtomicF64::new(r),
            g: AtomicF64::new(g),
            b: AtomicF64::new(b),
        }
    }
}
