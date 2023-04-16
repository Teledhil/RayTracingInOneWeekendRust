use crate::camera::Camera;
use crate::color::Color;
use crate::dielectric::Dielectric;
use crate::hittable_list::HittableList;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::metal::Metal;
use crate::point3::Point3;
use crate::sphere::Sphere;
use crate::vec3::{Length, Random, RandomRanged, Vec3};

pub struct Scene {
    camera: Camera,
    world: HittableList,
}

impl Scene {
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn world(&self) -> &HittableList {
        &self.world
    }

    pub fn three_spheres_custom_camera(aspect_ratio: f64) -> Self {
        // Camera
        let look_from = Point3::new(3.0, 3.0, 2.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vertical_fov = 20.0;
        let aperture = 2.0;
        let focus_distance = (look_from - look_at).length();

        let camera = Camera::new(
            look_from,
            look_at,
            up,
            vertical_fov,
            aspect_ratio,
            aperture,
            focus_distance,
        );

        // Materials
        let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let material_left = Box::new(Dielectric::new(1.5));
        let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

        let mut world = HittableList::default();
        let ground = Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        ));
        world.add(ground);

        let center_ball = Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            material_center,
        ));
        world.add(center_ball);

        let left_ball = Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        ));
        world.add(left_ball);

        // Hollow Glass
        let inner_hollow_ball = Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.45,
            material_left,
        ));
        let outer_hollow_ball = Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        ));
        world.add(inner_hollow_ball);
        world.add(outer_hollow_ball);

        Self { camera, world }
    }

    #[allow(illegal_floating_point_literal_pattern)]
    pub fn one_weekend(aspect_ratio: f64) -> Self {
        // Camera
        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vertical_fov = 20.0;
        let aperture = 0.1;
        let focus_distance = 10.0;

        let camera = Camera::new(
            look_from,
            look_at,
            up,
            vertical_fov,
            aspect_ratio,
            aperture,
            focus_distance,
        );

        // Materials
        let material_ground = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        let material_glass = Box::new(Dielectric::new(1.5));
        let material_lambertian = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        let material_metal = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

        // Ground
        let mut world = HittableList::default();
        let ground = Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground,
        ));
        world.add(ground);

        // Random Mini Balls
        for a in -11..11 {
            for b in -11..11 {
                const MINIBALL_RADIUS: f64 = 0.2;
                let center = Point3::new(
                    a as f64 + 0.9 * fastrand::f64(),
                    MINIBALL_RADIUS,
                    b as f64 + 0.9 * fastrand::f64(),
                );
                let ball_in_no_ball_area =
                    (center - Point3::new(4.0, MINIBALL_RADIUS, 0.0)).length() < 0.9;

                if !ball_in_no_ball_area {
                    let material: Box<dyn Material> = {
                        match fastrand::f64() {
                            0.0..=0.8 => {
                                // Diffuse
                                let albedo = Color::random() * Color::random();
                                Box::new(Lambertian::new(albedo))
                            }
                            0.8..=0.95 => {
                                // Metal
                                const ALBEDO_RANGE: std::ops::Range<f64> = 0.5..1.0;
                                let albedo = Color::random_ranged(&ALBEDO_RANGE);
                                const FUZZ_RANGE: std::ops::Range<f64> = 0.0..0.5;
                                let fuzz = f64::random_ranged(&FUZZ_RANGE);
                                Box::new(Metal::new(albedo, fuzz))
                            }
                            _ => {
                                // Glass
                                Box::new(Dielectric::new(1.5))
                            }
                        }
                    };
                    let miniball = Box::new(Sphere::new(center, MINIBALL_RADIUS, material));
                    world.add(miniball);
                }
            }
        }

        // Main Balls
        const MAINBALL_RADIUS: f64 = 1.0;
        let glassball = Box::new(Sphere::new(
            Point3::new(0.0, MAINBALL_RADIUS, 0.0),
            MAINBALL_RADIUS,
            material_glass,
        ));
        world.add(glassball);

        let brown_ball = Box::new(Sphere::new(
            Point3::new(-4.0, MAINBALL_RADIUS, 0.0),
            MAINBALL_RADIUS,
            material_lambertian,
        ));
        world.add(brown_ball);

        let metal_ball = Box::new(Sphere::new(
            Point3::new(4.0, MAINBALL_RADIUS, 0.0),
            MAINBALL_RADIUS,
            material_metal,
        ));
        world.add(metal_ball);

        Self { camera, world }
    }
}
