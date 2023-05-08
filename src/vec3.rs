use crate::dark_magic::{forward_ref_binop, forward_ref_unop};

#[cfg(not(feature = "simd"))]
use crate::scalar_vec3::ScalarVec3;

#[cfg(feature = "simd")]
use crate::simd_vec3::SimdVec3;

#[cfg(not(feature = "simd"))]
pub type PrivVec3<T> = ScalarVec3<T>;

#[cfg(feature = "simd")]
pub type PrivVec3<T> = SimdVec3<T>;

pub type Vec3 = PrivVec3<f64>;

// SquareRoot
//
pub trait SquareRoot {
    type Output;

    fn square_root(self) -> Self::Output;
}

// LengthSquared
//
pub trait LengthSquared {
    type Output;

    fn length_squared(self) -> Self::Output;
}

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
                self.length_squared().sqrt()
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
            type Output = Self;

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

// Cross
//
pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, other: Rhs) -> Self::Output;
}

impl Cross for Vec3 {
    type Output = Self;

    fn cross(self, other: Self) -> Self::Output {
        // e.x() = a.y()*b.z() - a.z()*b.y()
        let e0 = self.y().mul_add(other.z(), -self.z() * other.y());
        // e.y() = a.z()*b.x() - a.x()*b.z()
        let e1 = self.z().mul_add(other.x(), -self.x() * other.z());
        // e.z() = a.x()*b.y() - a.y()*b.x()
        let e2 = self.x().mul_add(other.y(), -self.y() * other.x());

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
            type RangeType = Self;

            fn random_ranged(range: &std::ops::Range<Self::RangeType>) -> Self {
                // range.start + (range.end - range.start) * fastrand::$t()
                (range.end - range.start).mul_add(fastrand::$t(), range.start)
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
                loop {
                    const RANGE:std::ops::Range<$t> = -1 as $t..1 as $t;
                    let candidate = Self::random_ranged(&RANGE);
                    if candidate.length_squared() < 1 as $t {
                        return candidate;
                    }
                }
            }
        }
    )*)
}
random_unit_sphere_impl! { f32 f64 }

// RandomInHemisphere
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
                loop {
                    const RANGE:std::ops::Range<$t> = -1 as $t..1 as $t;
                    let p = Self::new(
                        $t::random_ranged(&RANGE),
                        $t::random_ranged(&RANGE),
                        0 as $t);
                    if p.length_squared() < (1 as $t) {
                        return p;
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

// Zero
//
pub trait Zero {
    fn is_zero(&self) -> bool;
}

// NearZero
//
pub trait NearZero {
    fn is_near_zero(&self) -> bool;
}

macro_rules! near_zero_impl {
    ($($t:ty)*) => ($(
        impl NearZero for PrivVec3<$t> {
            fn is_near_zero(&self) -> bool {
                const ALMOST_ZERO: $t = 0.000000000001;
                self.x() < ALMOST_ZERO && self.y() < ALMOST_ZERO && self.z() < ALMOST_ZERO
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
            type Output = Self;

            fn reflect(self, normal: Self) -> Self::Output {
                let unit = self.unit();
                unit - 2 as $t * unit.dot(normal) * normal
            }
        }
        forward_ref_binop! { impl Reflect, reflect for PrivVec3<$t>, PrivVec3<$t> }
    )*)
}
reflect_impl! { f32 f64 }

// MulAdd
//
pub trait MulAdd<Rhs = Self> {
    fn mul_add(self, a: Rhs, b: Self) -> Self;
}

// Refract
//
pub fn refract(uv: Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * normal.mul_add(cos_theta, uv);
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
