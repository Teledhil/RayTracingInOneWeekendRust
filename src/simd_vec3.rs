use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::simd::{Simd, SimdFloat, StdFloat};

use crate::dark_magic::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use crate::vec3::{Dot, LengthSquared, MulAdd, SquareRoot, Zero};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SimdVec3<T: std::simd::SimdElement> {
    e: Simd<T, 4>,
}

impl<T: Copy + Default + std::simd::SimdElement> SimdVec3<T> {
    pub fn new(e1: T, e2: T, e3: T) -> Self {
        let e = Simd::from_array([e1, e2, e3, T::default()]);

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
        let e = Simd::from_array([e1, e2, e3, e1]);
        Self { e }
    }
}

// Default
//
impl<T: Default + Copy + std::simd::SimdElement> Default for SimdVec3<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

// v_0 = v_1 + v_2
macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add for SimdVec3<$t> {
            type Output = Self;

            fn add (self, other: Self) -> Self::Output {
                let e = self.e + other.e;

                Self::Output{e}
            }
        }
        forward_ref_binop! { impl Add, add for SimdVec3<$t>, SimdVec3<$t> }
    )*)
}
add_impl! { f32 f64 }

// v_0 += v_1
macro_rules! add_assign_impl {
    ($($t:ty)*) => ($(
        impl AddAssign for SimdVec3<$t> {
            fn add_assign(&mut self, other: SimdVec3<$t>) {
                self.e += other.e;
            }
        }
        forward_ref_op_assign! { impl AddAssign, add_assign for SimdVec3<$t>, SimdVec3<$t> }
    )*)
}
add_assign_impl! { f32 f64 }

// v_0 = v_1 - v_2
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Sub for SimdVec3<$t> {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                let e = self.e - other.e;

                Self::Output{e}
            }
        }
        forward_ref_binop! { impl Sub, sub for SimdVec3<$t>, SimdVec3<$t> }
    )*)
}
sub_impl! { f32 f64 }

// v_0 = -v_1
macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        impl Neg for SimdVec3<$t> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let e = -self.e;

                Self::Output{e}
            }
        }
        forward_ref_unop! { impl Neg, neg for SimdVec3<$t> }
    )*)
}
neg_impl! { f32 f64 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        // v_0 = v_1 * v_2
        impl Mul for SimdVec3<$t> {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                let e = self.e * other.e;

                Self::Output{e}
            }
        }
        forward_ref_binop! { impl Mul, mul for SimdVec3<$t>, SimdVec3<$t> }

        // v_0 = T * v_1
        impl Mul<SimdVec3<$t>> for $t {
            type Output = SimdVec3<$t>;

            fn mul(self, other: Self::Output) -> Self::Output {
                let simd_self = Self::Output::new(self, self, self);

                simd_self * other
            }
        }
        forward_ref_binop! { impl Mul, mul for $t, SimdVec3<$t> }

        // v_0 = v_1 * T
        impl Mul<$t> for SimdVec3<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> Self::Output {
                other * self
            }
        }
        forward_ref_binop! { impl Mul, mul for SimdVec3<$t>, $t }
    )*)
}
mul_impl! { f32 f64 }

// v_0 *= T
macro_rules! mul_assign_impl {
    ($($t:ty)*) => ($(
        impl MulAssign<$t> for SimdVec3<$t> {
            fn mul_assign(&mut self, value: $t) {
                let simd_value = Self::new(value, value, value);
                self.e *= simd_value.e;
            }
        }
        forward_ref_op_assign! { impl MulAssign, mul_assign for SimdVec3<$t>, $t }
    )*)
}
mul_assign_impl! { f32 f64 }

// v_0 = v_1 / T
macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div<$t> for SimdVec3<$t> {
            type Output = Self;

            fn div(self, value: $t) -> Self::Output {
                let simd_value = Self{e: Simd::from_array([value, value, value, 1.0])};
                let e = self.e / simd_value.e;

                Self::Output{e}
            }
        }
        forward_ref_binop! { impl Div, div for SimdVec3<$t>, $t }
    )*)
}
div_impl! { f32 f64 }

// v_0 /= T
macro_rules! div_assign_impl {
    ($($t:ty)*) => ($(
        impl DivAssign<$t> for SimdVec3<$t> {
            fn div_assign(&mut self, value: $t) {
                let simd_value = Self{e: Simd::from_array([value, value, value, 1.0])};
                self.e /= simd_value.e;
            }
        }
        forward_ref_op_assign! { impl DivAssign, div_assign for SimdVec3<$t>, $t }
    )*)
}
div_assign_impl! { f32 f64 }

// SquareRoot
//
macro_rules! square_root_impl {
    ($($t:ty)*) => ($(
        impl SquareRoot for SimdVec3<$t> {
            type Output = Self;

            fn square_root(self) -> Self::Output {
                let e = self.e.sqrt();
                Self::Output{e}
            }
        }
        forward_ref_unop! { impl SquareRoot, square_root for SimdVec3<$t> }
    )*)
}
square_root_impl! { f32 f64 }

// LengthSquared
//
macro_rules! length_squared_impl {
    ($($t:ty)*) => ($(
        impl LengthSquared for SimdVec3<$t> {
            type Output = $t;

            fn length_squared(self) -> Self::Output {
                let pow = self * self;
                pow.e.reduce_sum()
            }
        }
        forward_ref_unop! { impl LengthSquared, length_squared for SimdVec3<$t> }
    )*)
}
length_squared_impl! { f32 f64 }

// Dot
//
macro_rules! dot_impl {
    ($($t:ty)*) => ($(
        impl Dot for SimdVec3<$t> {
            type Output = $t;
            fn dot(self, other: Self) -> Self::Output {
                // a.x()*b.x() + a.y()*b.y() + a.z()*b.z()
                //
                // mul_add is faster in my Ryzen 9 5950X
                //
                //(self.e * other.e).reduce_sum()
                self.x().mul_add(other.x(), self.y().mul_add(other.y(), self.z()*other.z()))
            }
        }
        forward_ref_binop! { impl Dot, dot for SimdVec3<$t>, SimdVec3<$t> }
    )*)
}
dot_impl! { f32 f64 }

// Zero
//
macro_rules! zero_impl {
    ($($t:ty)*) => ($(
        impl Zero for SimdVec3<$t> {
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
        impl MulAdd for SimdVec3<$t> {
            fn mul_add(self, a: Self, b: Self) -> Self {
                let e = self.e.mul_add(a.e, b.e);

                Self{e}
            }
        }

        impl MulAdd<$t> for SimdVec3<$t> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_sum() {
        let v = Simd::from_array([1.0, 2.0, 3.0, f64::default()]);
        let res = v.reduce_sum();
        assert_eq!(res, 6.0);
    }

    #[test]
    fn test_mul() {
        let v = SimdVec3::<f64>::new(1.0, 2.0, 3.0);
        let res = v * v;
        let expected = SimdVec3::<f64>::new(1.0, 4.0, 9.0);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_length_squared() {
        let v = SimdVec3::<f64>::new(1.0, 2.0, 3.0);
        let res = v.length_squared();

        assert_eq!(14.0, res);
    }
}
