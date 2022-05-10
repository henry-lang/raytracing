use crate::Number;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Color {
    pub r: Number,
    pub g: Number,
    pub b: Number,
}

pub fn color(r: Number, g: Number, b: Number) -> Color {
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

            fn $fn(self, other: Number) -> Self::Output {
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
            fn $afn(&mut self, other: Number) {
                self.r $aop other;
                self.g $aop other;
                self.b $aop other;
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
