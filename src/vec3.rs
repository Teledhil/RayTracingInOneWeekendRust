use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

use crate::dark_magic::{forward_ref_binop, forward_ref_unop};

pub enum Coordinate {
    X,
    Y,
    Z,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PrivVec3<T> {
    e: [T; 3],
}

pub type Vec3 = PrivVec3<f64>;

impl<T: Copy> PrivVec3<T> {
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
impl<T: Default + Copy> Default for PrivVec3<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

// v_0 = v_1 + v_2
impl<T: Add + Add<Output = T> + Copy> Add for PrivVec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let e1 = self.e[0] + other.e[0];
        let e2 = self.e[1] + other.e[1];
        let e3 = self.e[2] + other.e[2];

        Self::Output::new(e1, e2, e3)
    }
}
forward_ref_binop! { impl Add, add for Vec3, Vec3 }

// v_0 += v_1
impl<T: Add + Add<Output = T> + Copy> AddAssign for PrivVec3<T> {
    fn add_assign(&mut self, other: Self) {
        let e1 = self.e[0] + other.e[0];
        let e2 = self.e[1] + other.e[1];
        let e3 = self.e[2] + other.e[2];

        *self = Self::new(e1, e2, e3);
    }
}

// v_0 = v_1 - v_2
impl<T: Sub + Sub<Output = T> + Copy> Sub for PrivVec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let e1 = self.e[0] - other.e[0];
        let e2 = self.e[1] - other.e[1];
        let e3 = self.e[2] - other.e[2];

        Self::Output::new(e1, e2, e3)
    }
}
forward_ref_binop! { impl Sub, sub for Vec3, Vec3 }

// v_0 = -v_1
impl<T: Neg + Neg<Output = T> + Copy> Neg for PrivVec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x(), -self.y(), -self.z())
    }
}
forward_ref_unop! { impl Neg, neg for Vec3 }

// v[]
impl<T> Index<Coordinate> for PrivVec3<T> {
    type Output = T;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        match coord {
            Coordinate::X => &self.e[0],
            Coordinate::Y => &self.e[1],
            Coordinate::Z => &self.e[2],
        }
    }
}

// v_0 = v_1 * v_2
impl<T: Mul + Mul<Output = T> + Copy> Mul for PrivVec3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let e1 = self.e[0] * other.e[0];
        let e2 = self.e[1] * other.e[1];
        let e3 = self.e[2] * other.e[2];

        Self::Output::new(e1, e2, e3)
    }
}
forward_ref_binop! { impl Mul, mul for Vec3, Vec3 }

// v_0 = v_1 * T
impl<T: Mul + Mul<Output = T> + Copy> Mul<T> for PrivVec3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let e1 = self.e[0] * other;
        let e2 = self.e[1] * other;
        let e3 = self.e[2] * other;

        Self::Output::new(e1, e2, e3)
    }
}

// v_0 = f64 * v_1
impl Mul<PrivVec3<f64>> for f64 {
    type Output = PrivVec3<f64>;

    fn mul(self, other: PrivVec3<f64>) -> Self::Output {
        let e1 = self * other.e[0];
        let e2 = self * other.e[1];
        let e3 = self * other.e[2];

        Self::Output::new(e1, e2, e3)
    }
}
forward_ref_binop! { impl Mul, mul for f64, Vec3 }

// v_0 = f32 * v_1
impl Mul<PrivVec3<f32>> for f32 {
    type Output = PrivVec3<f32>;

    fn mul(self, other: PrivVec3<f32>) -> Self::Output {
        let e1 = self * other.e[0];
        let e2 = self * other.e[1];
        let e3 = self * other.e[2];

        Self::Output::new(e1, e2, e3)
    }
}
forward_ref_binop! { impl Mul, mul for f32, PrivVec3<f32> }

// v_0 *= T
impl<T: Mul + Mul<Output = T> + Copy> MulAssign<T> for PrivVec3<T> {
    fn mul_assign(&mut self, value: T) {
        let e1 = self.e[0] * value;
        let e2 = self.e[1] * value;
        let e3 = self.e[2] * value;

        *self = Self::new(e1, e2, e3);
    }
}

// v_0 = v_1 / T
impl<T: Div + Div<Output = T> + Copy> Div<T> for PrivVec3<T> {
    type Output = PrivVec3<T>;

    fn div(self, value: T) -> Self::Output {
        let e1 = self.e[0] / value;
        let e2 = self.e[1] / value;
        let e3 = self.e[2] / value;

        Self::new(e1, e2, e3)
    }
}

// v_0 /= T
impl<T: Div + Div<Output = T> + Copy> DivAssign<T> for PrivVec3<T> {
    fn div_assign(&mut self, value: T) {
        let e1 = self.e[0] / value;
        let e2 = self.e[1] / value;
        let e3 = self.e[2] / value;

        *self = Self::new(e1, e2, e3);
    }
}

// Pow2
//
trait Pow2 {
    type Output;

    fn pow_2(self) -> Self::Output;
}

macro_rules! pow_2_impl {
    ($($t:ty)*) => ($(
        impl Pow2 for $t {
            type Output = $t;

            fn pow_2(self) -> $t { self.powi(2) }
        }
        forward_ref_unop! { impl Pow2, pow_2 for $t }
    )*)
}
pow_2_impl! { f32 f64 }

// SquareRoot
//
pub trait SquareRoot {
    type Output;

    fn square_root(self) -> Self::Output;
}

macro_rules! square_root_impl {
    ($($t:ty)*) => ($(
        impl SquareRoot for $t {
            type Output = $t;

            fn square_root(self) -> $t { self.sqrt() }
        }
        forward_ref_unop! { impl SquareRoot, square_root for $t }
    )*)
}
square_root_impl! { f32 f64 }
macro_rules! square_root_vec_impl {
    ($($t:ty)*) => ($(
        impl SquareRoot for PrivVec3<$t> {
            type Output = PrivVec3<$t>;

            fn square_root(self) -> PrivVec3<$t> {
                Self::new ( self.x().sqrt(), self.y().sqrt(), self.z().sqrt())
            }
        }
        forward_ref_unop! { impl SquareRoot, square_root for PrivVec3<$t> }
    )*)
}
square_root_vec_impl! { f32 f64 }

// LengthSquared
//
pub trait LengthSquared {
    type Output;

    fn length_squared(self) -> Self::Output;
}

macro_rules! length_squared_impl {
    ($($t:ty)*) => ($(
        impl LengthSquared for PrivVec3<$t> {
            type Output = $t;

            fn length_squared(self) -> Self::Output {
                self.x().pow_2() + self.y().pow_2() + self.z().pow_2()
            }
        }
        forward_ref_unop! { impl LengthSquared, length_squared for PrivVec3<$t> }
    )*)
}
length_squared_impl! { f32 f64 }

// Length
//
pub trait Length {
    type Output;

    fn length(self) -> Self::Output;
}

macro_rules! length_impl {
    ($($t:ty)*) => ($(
        impl Length for PrivVec3<$t> {
            type Output = $t;

            fn length(self) -> Self::Output {
                self.length_squared().square_root()
            }
        }
        forward_ref_unop! { impl Length, length for PrivVec3<$t> }
    )*)
}
length_impl! { f32 f64 }

// Unit
//
pub trait Unit {
    type Output;

    fn unit(self) -> Self::Output;
}

macro_rules! unit_impl {
    ($($t:ty)*) => ($(
        impl Unit for PrivVec3<$t> {
            type Output = PrivVec3<$t>;

            fn unit(self) -> Self::Output {
                self / self.length()
            }
        }
        forward_ref_unop! { impl Unit, unit for PrivVec3<$t> }
    )*)
}
unit_impl! { f32 f64 }

// Dot
//
pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, other: Rhs) -> Self::Output;
}

macro_rules! dot_impl {
    ($($t:ty)*) => ($(
        impl Dot for PrivVec3<$t> {
            type Output = $t;
            fn dot(self, other: Self) -> Self::Output {
                self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
            }
        }
        forward_ref_binop! { impl Dot, dot for PrivVec3<$t>, PrivVec3<$t> }
    )*)
}
dot_impl! { f32 f64 }

// Cross
//
pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, other: Rhs) -> Self::Output;
}

impl Cross for Vec3 {
    type Output = Self;

    fn cross(self, other: Self) -> Self::Output {
        let e0 = self.e[1] * other.e[2] - self.e[2] * other.e[1];
        let e1 = self.e[2] * other.e[0] - self.e[0] * other.e[2];
        let e2 = self.e[0] * other.e[1] - self.e[1] * other.e[0];

        Self::Output::new(e0, e1, e2)
    }
}
forward_ref_binop! { impl Cross, cross for Vec3, Vec3 }

// Random
//
pub trait Random {
    fn random() -> Self;
}

macro_rules! random_impl {
    ($($t:ident)*) => ($(
        impl Random for PrivVec3<$t> {
            fn random() -> Self {
                Self::new(fastrand::$t(), fastrand::$t(), fastrand::$t())
            }
        }
    )*)
}
random_impl! { f32 f64 }

// RandomRanged
//
pub trait RandomRanged {
    type RangeType;

    fn random_ranged(range: &std::ops::Range<Self::RangeType>) -> Self;
}
macro_rules! random_ranged_impl {
    ($($t:ident)*) => ($(
        impl RandomRanged for $t {
            type RangeType = $t;

            fn random_ranged(range: &std::ops::Range<Self::RangeType>) -> Self {
                range.start + (range.end - range.start) * fastrand::$t()
            }
        }

        impl RandomRanged for PrivVec3<$t> {
            type RangeType = $t;

            fn random_ranged(range: &std::ops::Range<Self::RangeType>) -> Self {
                let e1 :$t = $t::random_ranged(range);
                let e2 :$t = $t::random_ranged(range);
                let e3 :$t = $t::random_ranged(range);

                Self::new(e1, e2, e3)
            }
        }
    )*)
}
random_ranged_impl! { f32 f64 }

// RandomUnitSphere
//
pub trait RandomUnitSphere {
    fn random_unit_sphere() -> Self;
}

macro_rules! random_unit_sphere_impl {
    ($($t:ty)*) => ($(
        impl RandomUnitSphere for PrivVec3<$t> {
            fn random_unit_sphere() -> Self {
                let mut max_loop = 30;
                loop {
                    max_loop -= 1;
                    const RANGE:std::ops::Range<$t> = -1 as $t..1 as $t;
                    let candidate = Self::random_ranged(&RANGE);
                    if candidate.length_squared() < 1 as $t {
                        return candidate;
                    }
                    if max_loop <= 0 {
                        panic!("Failed to generate random unit sphere.\ncandidate: {candidate:#?}\nlength_square: {}",
                            candidate.length_squared());
                    }
                }
            }
        }
    )*)
}
random_unit_sphere_impl! { f32 f64 }

// RandomInhemisphere
//
pub trait RandomInHemisphere {
    fn random_in_hemisphere(normal: &Self) -> Self;
}

impl RandomInHemisphere for Vec3 {
    fn random_in_hemisphere(normal: &Self) -> Self {
        let random_unit_sphere = Self::random_unit_sphere();

        // In the same hemisphere as the normal
        if random_unit_sphere.dot(normal) > 0.0 {
            random_unit_sphere
        // In the opposite hemisphere
        } else {
            -random_unit_sphere
        }
    }
}

// RandomUnitDisk
//
pub trait RandomUnitDisk {
    fn random_unit_disk() -> Self;
}

macro_rules! random_unit_disk_impl {
    ($($t:ident)*) => ($(
        impl RandomUnitDisk for PrivVec3<$t> {
            fn random_unit_disk() -> Self {
                let mut max_loop = 30;
                loop {
                    max_loop -= 1;
                    const RANGE:std::ops::Range<$t> = -1 as $t..1 as $t;
                    let p = Self::new(
                        $t::random_ranged(&RANGE),
                        $t::random_ranged(&RANGE),
                        0 as $t);
                    if p.length_squared() < (1 as $t) {
                        return p;
                    }
                    if max_loop <= 0 {
                        panic!("Failed to generate random unit disk.\ncandidate: {p:#?}\nlength_square: {}",
                            p.length_squared());
                    }
                }
            }
        }
    )*)
}
random_unit_disk_impl! { f32 f64 }

// RandomUnitVector
//
pub trait RandomUnitVector {
    fn random_unit_vector() -> Self;
}

impl RandomUnitVector for Vec3 {
    fn random_unit_vector() -> Self {
        Self::random_unit_sphere().unit()
    }
}

pub trait Zero {
    fn is_zero(&self) -> bool;
}

macro_rules! near_zero_impl {
    ($($t:ty)*) => ($(
        impl Zero for PrivVec3<$t> {
            fn is_zero(&self) -> bool {
                const ZERO: $t = 0 as $t;
                self.e[0] == ZERO && self.e[1] == ZERO && self.e[2] == ZERO
            }
        }
    )*)
}
near_zero_impl! { f32 f64 }

pub trait NearZero {
    fn is_near_zero(&self) -> bool;
}

macro_rules! near_zero_impl {
    ($($t:ty)*) => ($(
        impl NearZero for PrivVec3<$t> {
            fn is_near_zero(&self) -> bool {
                const ALMOST_ZERO: $t = 0.000000000001;
                self.e[0] < ALMOST_ZERO && self.e[1] < ALMOST_ZERO && self.e[2] < ALMOST_ZERO
            }
        }
    )*)
}
near_zero_impl! { f32 f64 }

pub trait Reflect<Rhs = Self> {
    type Output;

    fn reflect(self, normal: Rhs) -> Self::Output;
}

macro_rules! reflect_impl {
    ($($t:ty)*) => ($(
        impl Reflect for PrivVec3<$t> {
            type Output = PrivVec3<$t>;

            fn reflect(self, normal: Self) -> Self::Output {
                let unit = self.unit();
                unit - 2 as $t * unit.dot(normal) * normal
            }
        }
        forward_ref_binop! { impl Reflect, reflect for PrivVec3<$t>, PrivVec3<$t> }
    )*)
}
reflect_impl! { f32 f64 }

pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(normal).min(1.0);
    let r_out_perp: Vec3 = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;

    r_out_perp + r_out_parallel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let v = Vec3::default();
        let f: f64 = 1.0;
        let _res = v * f;
    }

    #[test]
    fn test_mul_bis() {
        let v = Vec3::default();
        let f: f64 = 1.0;
        let _res = f * v;
    }

    #[test]
    fn test_dot() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(3.0, 2.0, 1.0);

        let res = u.dot(v);
        assert_eq!(res, 10.0);
    }

    #[test]
    fn test_cross() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(3.0, 2.0, 1.0);

        let res = u.cross(v);
        assert_eq!(res, Vec3::new(-4.0, 8.0, -4.0));
    }

    #[test]
    fn test_unit() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let unit = v.unit();
        assert_eq!(1.0, unit.length());

        let v_ref = &v;
        let unit_ref = v_ref.unit();
        assert_eq!(1.0, unit_ref.length());
    }

    #[test]
    fn test_random_unit_sphere() {
        for _ in 0..100 {
            let sphere = Vec3::random_unit_sphere();
            assert!(-1.0 <= sphere.x() && sphere.x() <= 1.0, "{}", sphere.x());
            assert!(-1.0 <= sphere.y() && sphere.y() <= 1.0, "{}", sphere.y());
            assert!(-1.0 <= sphere.z() && sphere.z() <= 1.0, "{}", sphere.z());
        }
    }
}
