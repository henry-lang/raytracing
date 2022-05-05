use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

macro_rules! operator_impl {
    ($(($t:ty, $tf:ty, $fn:ident, $op:tt, $at:ty, $atf:ty, $afn:ident, $aop:tt)),*) => {$(
        impl $t for Color {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                Self {
                    r: self.r $op other.r,
                    g: self.g $op other.g,
                    b: self.b $op other.b,
                }
            }
        }

        impl $tf for Color {
            type Output = Self;

            fn $fn(self, other: f64) -> Self::Output {
                Self {
                    r: self.r $op other,
                    g: self.g $op other,
                    b: self.b $op other,
                }
            }
        }

        impl $at for Color {
            fn $afn(&mut self, other: Self) {
                self.r $aop other.r;
                self.g $aop other.g;
                self.b $aop other.b;
            }
        }

        impl $atf for Color {
            fn $afn(&mut self, other: f64) {
                self.r $aop other;
                self.g $aop other;
                self.b $aop other;
            }
        }
    )*};
}

operator_impl!(
    (Add, Add<f64>, add, +, AddAssign, AddAssign<f64>, add_assign, +=),
    (Sub, Sub<f64>, sub, -, SubAssign, SubAssign<f64>, sub_assign, -=),
    (Mul, Mul<f64>, mul, *, MulAssign, MulAssign<f64>, mul_assign, *=),
    (Div, Div<f64>, div, /, DivAssign, DivAssign<f64>, div_assign, /=)
);
