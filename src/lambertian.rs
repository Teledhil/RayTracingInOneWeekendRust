use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{RandomInHemisphere, Vec3, Zero};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        // let mut scatter_direction = hit_record.normal() + Vec3::random_unit_vector();
        let mut scatter_direction = Vec3::random_in_hemisphere(hit_record.normal());
        if scatter_direction.is_zero() {
            scatter_direction = *hit_record.normal();
        }

        let scattered = Ray::new(hit_record.p(), scatter_direction);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
