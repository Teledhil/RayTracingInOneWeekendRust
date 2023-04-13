use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}