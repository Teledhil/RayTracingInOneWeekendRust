use crate::color::{Color, WHITE};
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{refract, Dot, Reflect, Unit};

#[derive(Clone)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    const ATTENUATION: Color = WHITE;
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cos_theta: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1_f64 - ref_idx) / (1_f64 + ref_idx);
        r0 = r0 * r0;

        r0 + (1_f64 - r0) * (1_f64 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = match hit_record.front_face() {
            true => 1.0 / self.ir,
            false => self.ir,
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction).dot(hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let must_reflect = Self::reflectance(cos_theta, refraction_ratio) > fastrand::f64();
        let direction = match cannot_refract || must_reflect {
            true => unit_direction.reflect(hit_record.normal()),
            false => refract(&unit_direction, hit_record.normal(), refraction_ratio),
        };

        let scattered = Ray::new(hit_record.p(), direction);

        Some((scattered, Self::ATTENUATION))
    }
}
