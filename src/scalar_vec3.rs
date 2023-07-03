use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::dark_magic::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use crate::vec3::{Dot, LengthSquared, MulAdd, SquareRoot, Zero};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ScalarVec3<T> {
    e: [T; 3],
}

impl<T: Copy> ScalarVec3<T> {
    pub fn new(e1: T, e2: T, e3: T) -> Self {
        let e: [T; 3] = [e1, e2, e3];

        Self { e }
    }

    pub fn x(&self) -> T {
        self.e[0]
    }

    pub fn y(&self) -> T {
        self.e[1]
    }

    pub fn z(&self) -> T {
        self.e[2]
    }

    pub const fn const_new(e1: T, e2: T, e3: T) -> Self {
        let e: [T; 3] = [e1, e2, e3];
        Self { e }
    }
}

// Default
//
impl<T: Default + Copy> Default for ScalarVec3<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

// v_0 = v_1 + v_2
macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add for ScalarVec3<$t> {
            type Output = Self;

            fn add (self, other: Self) -> Self::Output {
                let e1 = self.x() + other.x();
                let e2 = self.y() + other.y();
                let e3 = self.z() + other.z();

                Self::Output::new(e1, e2, e3)
            }
        }
        forward_ref_binop! { impl Add, add for ScalarVec3<$t>, ScalarVec3<$t> }
    )*)
}
add_impl! { f32 f64 }

// v_0 += v_1
macro_rules! add_assign_impl {
    ($($t:ty)*) => ($(
        impl AddAssign for ScalarVec3<$t> {
            fn add_assign(&mut self, other: ScalarVec3<$t>) {
                let e1 = self.x() + other.x();
                let e2 = self.y() + other.y();
                let e3 = self.z() + other.z();

                *self = Self::new(e1, e2, e3);
            }
        }
        forward_ref_op_assign! { impl AddAssign, add_assign for ScalarVec3<$t>, ScalarVec3<$t> }
    )*)
}
add_assign_impl! { f32 f64 }

// v_0 = v_1 - v_2
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Sub for ScalarVec3<$t> {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                let e1 = self.x() - other.x();
                let e2 = self.y() - other.y();
                let e3 = self.z() - other.z();

                Self::Output::new(e1, e2, e3)
            }
        }
        forward_ref_binop! { impl Sub, sub for ScalarVec3<$t>, ScalarVec3<$t> }
    )*)
}
sub_impl! { f32 f64 }

// v_0 = -v_1
macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        impl Neg for ScalarVec3<$t> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::Output::new(-self.x(), -self.y(), -self.z())
            }
        }
        forward_ref_unop! { impl Neg, neg for ScalarVec3<$t> }
    )*)
}
neg_impl! { f32 f64 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        // v_0 = v_1 * v_2
        impl Mul for ScalarVec3<$t> {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                let e1 = self.x() * other.x();
                let e2 = self.y() * other.y();
                let e3 = self.z() * other.z();

                Self::Output::new(e1, e2, e3)
            }
        }
        forward_ref_binop! { impl Mul, mul for ScalarVec3<$t>, ScalarVec3<$t> }

        // v_0 = T * v_1
        impl Mul<ScalarVec3<$t>> for $t {
            type Output = ScalarVec3<$t>;

            fn mul(self, other: Self::Output) -> Self::Output {
                let e1 = self * other.x();
                let e2 = self * other.y();
                let e3 = self * other.z();

                Self::Output::new(e1, e2, e3)
            }
        }
        forward_ref_binop! { impl Mul, mul for $t, ScalarVec3<$t> }

        // v_0 = v_1 * T
        impl Mul<$t> for ScalarVec3<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> Self::Output {
                other * self
            }
        }
        forward_ref_binop! { impl Mul, mul for ScalarVec3<$t>, $t }
    )*)
}
mul_impl! { f32 f64 }

// v_0 *= T
macro_rules! mul_assign_impl {
    ($($t:ty)*) => ($(
        impl MulAssign<$t> for ScalarVec3<$t> {
            fn mul_assign(&mut self, value: $t) {
                let e1 = self.x() * value;
                let e2 = self.y() * value;
                let e3 = self.z() * value;

                *self = Self::new(e1, e2, e3);
            }
        }
        forward_ref_op_assign! { impl MulAssign, mul_assign for ScalarVec3<$t>, $t }
    )*)
}
mul_assign_impl! { f32 f64 }

// v_0 = v_1 / T
macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div<$t> for ScalarVec3<$t> {
            type Output = Self;

            fn div(self, value: $t) -> Self::Output {
                let e1 = self.x() / value;
                let e2 = self.y() / value;
                let e3 = self.z() / value;

                Self::new(e1, e2, e3)
            }
        }
        forward_ref_binop! { impl Div, div for ScalarVec3<$t>, $t }
    )*)
}
div_impl! { f32 f64 }

// v_0 /= T
macro_rules! div_assign_impl {
    ($($t:ty)*) => ($(
        impl DivAssign<$t> for ScalarVec3<$t> {
            fn div_assign(&mut self, value: $t) {
                let e1 = self.x() / value;
                let e2 = self.y() / value;
                let e3 = self.z() / value;

                *self = Self::new(e1, e2, e3);
            }
        }
        forward_ref_op_assign! { impl DivAssign, div_assign for ScalarVec3<$t>, $t }
    )*)
}
div_assign_impl! { f32 f64 }

// SquareRoot
//
macro_rules! square_root_impl {
    ($($t:ty)*) => ($(
        impl SquareRoot for ScalarVec3<$t> {
            type Output = Self;

            fn square_root(self) -> Self::Output {
                Self::Output::new ( self.x().sqrt(), self.y().sqrt(), self.z().sqrt())
            }
        }
        forward_ref_unop! { impl SquareRoot, square_root for ScalarVec3<$t> }
    )*)
}
square_root_impl! { f32 f64 }

// LengthSquared
//
macro_rules! length_squared_impl {
    ($($t:ty)*) => ($(
        impl LengthSquared for ScalarVec3<$t> {
            type Output = $t;

            fn length_squared(self) -> Self::Output {
                // x^2 + y^2 + z^2
                self.x().mul_add(self.x(), self.y().mul_add(self.y(), self.z()*self.z()))
            }
        }
        forward_ref_unop! { impl LengthSquared, length_squared for ScalarVec3<$t> }
    )*)
}
length_squared_impl! { f32 f64 }

// Dot
//
macro_rules! dot_impl {
    ($($t:ty)*) => ($(
        impl Dot for ScalarVec3<$t> {
            type Output = $t;
            fn dot(self, other: Self) -> Self::Output {
                // a.x()*b.x() + a.y()*b.y() + a.z()*b.z()
                self.x().mul_add(other.x(), self.y().mul_add(other.y(), self.z()*other.z()))
            }
        }
        forward_ref_binop! { impl Dot, dot for ScalarVec3<$t>, ScalarVec3<$t> }
    )*)
}
dot_impl! { f32 f64 }

// Zero
//
macro_rules! zero_impl {
    ($($t:ty)*) => ($(
        impl Zero for ScalarVec3<$t> {
            fn is_zero(&self) -> bool {
                self == &Self::default()
            }
        }
    )*)
}
zero_impl! { f32 f64 }

// MulAdd
//
macro_rules! mul_add_impl {
    ($($t:ty)*) => ($(
        impl MulAdd for ScalarVec3<$t> {
            fn mul_add(self, a: Self, b: Self) -> Self {
                let e0 = self.x().mul_add(a.x(), b.x());
                let e1 = self.y().mul_add(a.y(), b.y());
                let e2 = self.z().mul_add(a.z(), b.z());

                Self::new(e0, e1, e2)

            }
        }

        impl MulAdd<$t> for ScalarVec3<$t> {
            fn mul_add(self, a: $t, b: Self) -> Self {
                let e0 = self.x().mul_add(a, b.x());
                let e1 = self.y().mul_add(a, b.y());
                let e2 = self.z().mul_add(a, b.z());

                Self::new(e0, e1, e2)
            }
        }
    )*)
}
mul_add_impl! { f32 f64 }
