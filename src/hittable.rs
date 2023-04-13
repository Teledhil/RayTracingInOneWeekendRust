use std::ops::Range;
use std::rc::Rc;

use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Dot;
use crate::vec3::Vec3;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
        t: f64,
        r: &Ray,
    ) -> Self {
        let (front_face, normal) = Self::set_face_normal(r, outward_normal);
        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }

    fn set_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        match r.direction().dot(outward_normal) < 0_f64 {
            true => {
                let front_face = true;
                let normal = outward_normal;

                (front_face, normal)
            }
            false => {
                let front_face = false;
                let normal = -outward_normal;

                (front_face, normal)
            }
        }
    }

    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}
