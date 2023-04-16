use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}
