mod camera;
mod color;
mod geometry;
mod scene;
use color::Color;
use geometry::{Direction, Location};
use scene::*;

const ANTI_ALIASING: u32 = 100;

fn main() {
    // Create a test image
    let mut my_image: image::RgbImage = image::RgbImage::new(400, 300);

    {
        println!("Creating image...");

        // Create a simple object
        let mut objects = Vec::new();
        objects.push(objects::Sphere {
            origin: Location::new(1.0, 0.0, 0.0),
            radius: 0.5,
        });
        objects.push(objects::Sphere {
            origin: Location::new(1.0, 0.0, -100.5),
            radius: 100.0,
        });

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
                let (_distance, normal) = objects.iter().fold((f64::MAX, None), |acc, sphere| {
                    if let Some(hit) = sphere.get_hits(&ray) {
                        // Check distance
                        if hit.distance < acc.0 && hit.distance > 0.0 {
                            (hit.distance, Some(hit.normal))
                        } else {
                            acc
                        }
                    } else {
                        acc
                    }
                });
                color = color
                    + if let Some(hit_normal) = normal {
                        let hit_normal = hit_normal.as_slice();
                        // Calculate color based on the normal of the hit
                        Color::new(-hit_normal[1] / 2.0 + 0.5, 0.0, 0.0)
                            + Color::new(0.0, hit_normal[2] / 2.0 + 0.5, 0.0)
                            + Color::new(0.0, 0.0, -hit_normal[0] / 2.0 + 0.5)
                    } else {
                        let t = ray.direction.norm().z() / 2.0 + 0.5;
                        Color::white() * (1.0 - t) + Color::blue() * t
                    };
            }
            color = color / ANTI_ALIASING as f64;
            pixel.0[0] = (color.r() * 255.9999) as u8;
            pixel.0[1] = (color.g() * 255.9999) as u8;
            pixel.0[2] = (color.b() * 255.9999) as u8;
            if u == 0 {
                progress.inc();
            }
        }
        println!();
    }
    println!("Saving image ...");
    my_image.save("test.png").unwrap();
}
