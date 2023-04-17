use std::sync::{mpsc, Arc, Mutex};
use std::thread;

mod buffer;
mod camera;
mod color;
mod dark_magic;
mod dielectric;
mod hit_record;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod point3;
mod ray;
pub mod scene;
mod sphere;
mod vec3;

use crate::buffer::Buffer;
use crate::color::{Color, BLACK, WHITE};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::{mul_add, SquareRoot, Unit};

fn ray_color(r: &Ray, world: &HittableList, depth: i8) -> Color {
    if depth <= 0 {
        return BLACK;
    }

    let t_range = 0.001..std::f64::INFINITY;
    if let Some(hit_record) = world.hit(r, &t_range) {
        //if hit_record.t() < 0.001 {
        //    // Ray hit too close
        //    return BLACK;
        //}
        if let Some((scattered, attenuation)) = hit_record.material().scatter(r, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return BLACK;
    }

    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);

    // (1.0 - t) * WHITE + t * Color::new(0.5, 0.7, 1.0)
    mul_add(&WHITE, 1.0 - t, &(t * Color::new(0.5, 0.7, 1.0)))
}

pub fn rtx(
    scene: Scene,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: u16,
    depth: i8,
) -> anyhow::Result<Buffer> {
    let buffer = Buffer::new(image_width, image_height);

    let (sender, receiver) = mpsc::channel();

    for height in (0..image_height).rev() {
        if let Err(e) = sender.send(height) {
            panic!("Failed to send line number {height}: {e}");
        }
    }
    drop(sender);

    let receiver = Arc::new(Mutex::new(receiver));
    let num_threads = std::thread::available_parallelism()?.get();
    println!("Spawning {num_threads} threads");
    thread::scope(|s| {
        let mut threads = Vec::new();

        // Spawn thread workers
        for _ in 0..num_threads {
            let thread = s.spawn(|| {
                let mut lines_drawed = 0;
                loop {
                    let message = receiver.lock().unwrap().recv();
                    if let Ok(height) = message {
                        let mut line = buffer.get_line();
                        rtx_line(
                            &scene,
                            image_width,
                            image_height,
                            samples_per_pixel,
                            depth,
                            &mut line,
                            height,
                        );
                        buffer.push_line(height, line);
                        lines_drawed += 1;
                    } else {
                        return lines_drawed;
                    }
                }
            });
            threads.push(thread);
        }

        // Collect result from the thread workers
        let mut stats = Vec::new();
        for thread in threads {
            match thread.join() {
                Ok(lines_drawed) => stats.push(lines_drawed),
                Err(e) => println!("Thread failed with {e:#?}"),
            }
        }
        let str_stats = stats
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Lines drawed per thread: {}", str_stats);
    });

    Ok(buffer)
}

fn rtx_line(
    scene: &Scene,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: u16,
    depth: i8,
    line: &mut Vec<Color>,
    height: usize,
) {
    for width in 0..image_width {
        let mut pixel_color = BLACK;
        for _sample in 0..samples_per_pixel {
            let u: f64 = (width as f64 + fastrand::f64()) / (image_width as f64 - 1_f64);
            let v: f64 = (height as f64 + fastrand::f64()) / (image_height as f64 - 1_f64);

            let ray = scene.camera().get_ray(u, v);
            let sample_pixel_color = ray_color(&ray, scene.world(), depth);
            pixel_color += sample_pixel_color;
        }

        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let calibrated_pixel_color = (pixel_color / samples_per_pixel as f64).square_root();
        line.push(calibrated_pixel_color);
    }
}
