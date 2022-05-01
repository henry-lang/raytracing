use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

macro_rules! operator_impl {
    ($(($t:ty, $fn:ident, $op:tt, $at:ty, $afn:ident, $aop:tt)),*) => {$(
        impl $t for Vector3 {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                Self {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                    z: self.z $op other.z,
                }
            }
        }

        impl $at for Vector3 {
            fn $afn(&mut self, other: Self) {
                self.x $aop other.x;
                self.y $aop other.y;
                self.z $aop other.z;
            }
        }
    )*};
}

operator_impl!(
    (Add, add, +, AddAssign, add_assign, +=),
    (Sub, sub, -, SubAssign, sub_assign, -=),
    (Mul, mul, *, MulAssign, mul_assign, *=),
    (Div, div, /, DivAssign, div_assign, /=)
);
