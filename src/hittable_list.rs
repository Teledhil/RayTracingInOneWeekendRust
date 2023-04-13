use std::ops::Range;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn push(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        let wrapped_sphere = Box::new(sphere);
        self.push(wrapped_sphere);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_range: &Range<f64>) -> Option<HitRecord> {
        let mut closest_hit_record = None;
        let mut closest_t = t_range.end;
        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(r, t_range) {
                if hit_record.t() < closest_t {
                    closest_t = hit_record.t();
                    closest_hit_record = Some(hit_record);
                }
            }
        }

        closest_hit_record
    }
}
