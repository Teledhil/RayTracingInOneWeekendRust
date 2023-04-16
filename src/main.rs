use rust_ray_tracer::rtx;
use rust_ray_tracer::world::World;

fn main() -> anyhow::Result<()> {
    fastrand::seed(1337);

    const SAMPLES_PER_PIXEL: u16 = 500;
    const DEPTH: i8 = 20;

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 1920;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let world = World::one_weekend(ASPECT_RATIO);

    let buffer = rtx(world, IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, DEPTH)?;

    buffer.save("image.ppm")?;

    Ok(())
}
