use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Dot, RandomUnitSphere, Reflect, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.direction().reflect(hit_record.normal());
        let fuzzines = self.fuzz * Vec3::random_unit_sphere();
        let scattered = Ray::new(hit_record.p(), reflected + fuzzines);
        let attenuation = self.albedo;

        if scattered.direction().dot(hit_record.normal()) > 0_f64 {
            return Some((scattered, attenuation));
        }

        None
    }
}
