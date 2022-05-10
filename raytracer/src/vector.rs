use crate::{
    color::{color, Color},
    Number,
};
use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: Number,
    pub y: Number,
    pub z: Number,
}

pub fn vector3(x: Number, y: Number, z: Number) -> Vector3 {
    Vector3 { x, y, z }
}

impl Vector3 {
    pub fn random(rand: &mut impl Rng) -> Self {
        vector3(
            rand.gen_range(-1.0..1.0),
            rand.gen_range(-1.0..1.0),
            rand.gen_range(-1.0..1.0),
        )
    }

    pub fn random_in_unit_sphere(rand: &mut impl Rng) -> Self {
        loop {
            let random = Self::random(rand);

            if random.length_squared() >= 1.0 {
                continue;
            }

            return random;
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * self.dot(normal) * 2.0
    }

    pub fn random_normalized(rand: &mut impl Rng) -> Self {
        Self::random_in_unit_sphere(rand)
    }

    pub fn near_zero(&self) -> bool {
        let tolerance = 0.00000001;

        self.x.abs() < tolerance && self.y.abs() < tolerance && self.z.abs() < tolerance
    }

    pub fn dot(&self, other: &Self) -> Number {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> Number {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> Number {
        self.length_squared().sqrt()
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

impl Into<Color> for Vector3 {
    fn into(self) -> Color {
        color(self.x, self.y, self.z)
    }
}

macro_rules! operator_impl {
    ($(($t:ty, $tf:ty, $fn:ident, $op:tt, $at:ty, $atf:ty, $afn:ident, $aop:tt)),*) => {$(
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

        impl $tf for Vector3 {
            type Output = Self;

            fn $fn(self, other: Number) -> Self::Output {
                Self {
                    x: self.x $op other,
                    y: self.y $op other,
                    z: self.z $op other,
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

        impl $atf for Vector3 {
            fn $afn(&mut self, other: Number) {
                self.x $aop other;
                self.y $aop other;
                self.z $aop other;
            }
        }
    )*};
}

operator_impl!(
    (Add, Add<Number>, add, +, AddAssign, AddAssign<Number>, add_assign, +=),
    (Sub, Sub<Number>, sub, -, SubAssign, SubAssign<Number>, sub_assign, -=),
    (Mul, Mul<Number>, mul, *, MulAssign, MulAssign<Number>, mul_assign, *=),
    (Div, Div<Number>, div, /, DivAssign, DivAssign<Number>, div_assign, /=)
);
