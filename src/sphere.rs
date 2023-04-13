use std::ops::Range;
use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::{Dot, LengthSquared};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: &Range<f64>) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0_f64 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest point that lies in the t_range
        let mut root = (-half_b - sqrtd) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        // Hit!
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let hit_record = HitRecord::new(p, outward_normal, self.material.clone(), t, r);

        Some(hit_record)
    }
}
