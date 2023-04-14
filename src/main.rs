use std::fs::File;
use std::io::{stdout, Write};

use termion::raw::IntoRawMode;

use rust_ray_tracer::color::{Color, BLACK, WHITE};
use rust_ray_tracer::hittable::Hittable;
use rust_ray_tracer::hittable_list::HittableList;
use rust_ray_tracer::ray::Ray;
use rust_ray_tracer::vec3::{mul_add, SquareRoot, Unit};
use rust_ray_tracer::world::World;

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

    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    mul_add(&WHITE, 1.0 - t, &(t * Color::new(0.5, 0.7, 1.0)))
}

fn main() -> anyhow::Result<()> {
    fastrand::seed(1337);

    const SAMPLES_PER_PIXEL: u16 = 500;
    const DEPTH: i8 = 20;

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let world = World::one_weekend(ASPECT_RATIO);

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut f = File::create("image.ppm")?;
    write!(&mut f, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n256\n")?;

    for height in (0..IMAGE_HEIGHT).rev() {
        write!(
            stdout,
            "{}\rDrawing ({}/{IMAGE_HEIGHT})",
            termion::clear::CurrentLine,
            IMAGE_HEIGHT - height + 1
        )?;
        stdout.flush().unwrap();

        for width in 0..IMAGE_WIDTH {
            let mut pixel_color = BLACK;
            for _sample in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (width as f64 + fastrand::f64()) / (IMAGE_WIDTH as f64 - 1_f64);
                let v: f64 = (height as f64 + fastrand::f64()) / (IMAGE_HEIGHT as f64 - 1_f64);

                let ray = world.camera().get_ray(u, v);
                let sample_pixel_color = ray_color(&ray, world.world(), DEPTH);
                pixel_color += sample_pixel_color;
            }

            let calibrated_pixel_color = (pixel_color / SAMPLES_PER_PIXEL as f64).square_root();
            write!(f, "{calibrated_pixel_color}")?;
        }
    }

    writeln!(stdout, "{}\rDone\r", termion::clear::CurrentLine)?;
    stdout.flush().unwrap();

    Ok(())
}
