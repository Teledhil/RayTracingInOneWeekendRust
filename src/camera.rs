use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::{mul_add, Cross, RandomUnitDisk, Unit, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        vertical_fov: f64, // Vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2_f64).tan();
        let viewport_height: f64 = 2_f64 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = up.cross(w).unit();
        let v = w.cross(u);

        let origin: Point3 = look_from;
        let horizontal: Vec3 = focus_distance * viewport_width * u;
        let vertical: Vec3 = focus_distance * viewport_height * v;
        let lower_left_corner: Vec3 = origin - (horizontal + vertical) / 2_f64 - focus_distance * w;

        let lens_radius = aperture / 2_f64;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_unit_disk();

        // ray_origin = self.origin + offset
        // offset = self.u * rd.x() + self.v * rd.y()
        // ray_origin = self.origin + self.u * rd.x() + self.v * rd.y()
        let origin = mul_add(&self.u, rd.x(), &mul_add(&self.v, rd.y(), &self.origin));

        // direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - origin
        let direction = mul_add(
            &self.horizontal,
            s,
            &mul_add(&self.vertical, t, &(self.lower_left_corner - origin)),
        );

        Ray::new(origin, direction)
    }
}
