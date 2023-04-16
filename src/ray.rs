use crate::point3::Point3;
use crate::vec3::{mul_add, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        // r(t) = origin + direction * t
        // self.origin + t * self.direction
        mul_add(&self.direction, t, &self.origin)
    }
}
