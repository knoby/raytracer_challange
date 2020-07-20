mod camera;
mod color;
mod geometry;
mod scene;
use color::Color;
use geometry::{Direction, Location};
use scene::World;

const ANTI_ALIASING: u32 = 100;

fn main() {
    // Create a test image
    let mut my_image: image::RgbImage = image::RgbImage::new(800, 600);

    {
        println!("Creating image...");

        // Create a sample world
        let world = World::sample_world();

        let mut progress = progress_bar::progress_bar::ProgressBar::new(my_image.height() as usize);
        progress.set_action(
            "Raytracing",
            progress_bar::color::Color::Blue,
            progress_bar::color::Style::Bold,
        );

        // Define the camera
        let mut cam = camera::Camera::new(
            Location::origin(),
            Direction::new(1.0, 0.0, 0.0),
            my_image.width(),
            my_image.height(),
            1.0,
        );

        // Iterate over the image
        for (u, v, pixel) in my_image.enumerate_pixels_mut() {
            // Check all objects for a hit
            let mut color = Color::black();
            for _ in 0..ANTI_ALIASING {
                // Get a ray to the pixel from the cam
                let ray = cam.get_ray(u, v).unwrap();
                color = color + world.get_ray_color(ray, 0);
            }
            // Correct for anti aliasing
            color = color / ANTI_ALIASING as f64;
            // Do some gamma corrections
            pixel.0[0] = (color.r().sqrt() * 255.9999) as u8;
            pixel.0[1] = (color.g().sqrt() * 255.9999) as u8;
            pixel.0[2] = (color.b().sqrt() * 255.9999) as u8;
            if u == 0 {
                progress.inc();
            }
        }
        println!();
    }
    println!("Saving image ...");
    my_image.save("test.png").unwrap();
}
