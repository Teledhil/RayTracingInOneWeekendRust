use std::ops::Range;

use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}
