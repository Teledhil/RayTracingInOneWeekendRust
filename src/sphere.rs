use std::ops::Range;

use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::{Dot, LengthSquared};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: &Range<f64>) -> Option<HitRecord> {
        // Given a sphere centered in C = (Cx, Cy, Cz) and radius r, the points in the sphere
        // P = (Px, Py, Pz):
        //   (Px -C//x)^2 + (Py - Cy)^2 + (Pz - Cz)^2 = r^2
        //   (P - C) · (P - C) = r^2
        //
        // with P being a ray, P(t)  = A + B*t:
        //
        //  (A + B*t - C) · (A + B*t - C) = r^2
        //  (B*t + A - C) · (B*t + A - C) = r^2
        //
        // Quadratic equation:
        //   f(t) = a*t^2 + b*t + c = 0
        //   r = (-b +- sqrt(b^2 - 4*a*c)) / (2*a)
        //   r = (-(2*half_b) +- sqrt((2*half_b)^2 - 4*a*c)) / (2*a)
        //   r = (-(2*half_b) +- sqrt(4*(half_b^2 - a*c)) / (2*a)
        //   r = (-(2*half_b) +- 2*sqrt(half_b^2 - a*c)) / (2*a)
        //   r = (-half_b +- sqrt(half_b^2 - a*c)) / a
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        // half_b^2 - a*c
        let discriminant = half_b.mul_add(half_b, -a * c);
        if discriminant < 0_f64 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest point that lies in the t_range
        // |------ r1 ---- r2 -----|
        // |---ttttttttt-----------|
        // r1 = (-half_b - sqrt(half_b^2 - a*c)) / a
        let mut root = (-half_b - sqrtd) / a;
        if !t_range.contains(&root) {
            // r2 = (-half_b + sqrt(half_b^2 - a*c)) / a
            root = (-half_b + sqrtd) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        // Hit in range!
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let hit_record = HitRecord::new(p, outward_normal, &*self.material, t, r);

        Some(hit_record)
    }
}
