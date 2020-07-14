mod color;
mod geometry;
mod scene;
use color::Color;
use geometry::{Direction, Location};
use scene::*;

fn main() {
    // Create a test image
    let mut my_image: image::RgbImage = image::RgbImage::new(1368, 768);

    // Get the size of the image
    let scale_x = my_image.width();
    let scale_y = my_image.height();

    {
        println!("Creating image...");

        // Create a simple object
        let sphere = objects::Sphere {
            origin: Location::new(1.0, 0.0, 0.0),
            radius: 0.2,
        };

        let mut progress = progress_bar::progress_bar::ProgressBar::new(scale_y as usize);
        progress.set_action(
            "Raytracing",
            progress_bar::color::Color::Blue,
            progress_bar::color::Style::Bold,
        );

        // Define the camera
        let cam_origin = Location::new(0.0, 0.0, 0.0);
        let cam_direction = Direction::new(1.0, 0.0, 0.0);

        // Define the viewport
        // In this case the height is 1 and the width depends on the image aspect ratio
        let aspect_ratio = scale_y as f64 / scale_x as f64;
        let viewport_height = 1.0;
        let viewport_widht = viewport_height / aspect_ratio;
        let focal_length = 1.0; // Distance from camera Origitn to viewport
        let horizontal = Direction::new(0.0, -1.0, 0.0).norm();
        let vertical = Direction::new(0.0, 0.0, -1.0).norm();
        let viewport_top_left = cam_origin + cam_direction.norm() * focal_length
            - horizontal * viewport_widht / 2.0
            - vertical * viewport_height / 2.0;
        // Some helper directions

        // Iterate over the image
        for (x, y, pixel) in my_image.enumerate_pixels_mut() {
            // Calcualte the ray to the pixel center
            let ray = Ray {
                origin: cam_origin,
                direction: ((viewport_top_left
                    + horizontal / scale_x as f64 * x as f64 * viewport_widht
                    + vertical / scale_y as f64 * y as f64 * viewport_height)
                    - cam_origin),
            };

            // If its a hit draw red, otherwise the background color
            let color = if let Some(hits) = sphere.get_hits(&ray) {
                if hits[0].distance > 0.0 {
                    Color::red()
                } else {
                    let t = (ray.direction.norm().z() + 0.5) / viewport_height;
                    Color::white() * (1.0 - t) + Color::blue() * t
                }
            } else {
                let t = (ray.direction.norm().z() + 0.5) / viewport_height;
                Color::white() * (1.0 - t) + Color::blue() * t
            };
            pixel.0[0] = (color.r() * 255.0) as u8;
            pixel.0[1] = (color.g() * 255.0) as u8;
            pixel.0[2] = (color.b() * 255.0) as u8;
            if x == scale_x as u32 - 1 {
                progress.inc();
            }
        }
        println!();
    }
    println!("Saving image ...");
    my_image.save("test.png").unwrap();
}
