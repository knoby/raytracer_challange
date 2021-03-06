mod camera;
mod color;
mod geometry;
mod scene;
mod threadpool;
use color::Color;
use geometry::{Direction, Location};
use scene::World;

const ANTI_ALIASING: u32 = 1000;

fn main() {
    let start_time = std::time::Instant::now();
    // Create a test image
    let mut my_image: image::RgbImage = image::RgbImage::new(800, 600);

    {
        println!("Creating image...");

        // Create a sample world
        let world = World::sample_world();

        // Define the camera
        let mut cam = camera::Camera::new(
            Location::origin(),
            Direction::new(1.0, 0.0, 0.0),
            my_image.width(),
            my_image.height(),
            1.0,
        );

        // Wrap the world struct in a Arc to be able to send it to the threadpool
        let world = std::sync::Arc::new(world);
        // Create a sender/reciver pair for returning colors to the main function
        let (tx_color, rx_color) = std::sync::mpsc::channel::<(usize, usize, Color)>();
        // Create a field to store the colors
        let mut colors: Vec<Vec<Color>> = Vec::with_capacity(my_image.width() as usize);
        for _ in 0..my_image.width() {
            let mut color_col = Vec::with_capacity(my_image.height() as usize);
            for _ in 0..my_image.height() {
                color_col.push(Color::black());
            }
            colors.push(color_col);
        }
        {
            // Check number of cores
            let cores = num_cpus::get();
            println!(
                "Found {} cores. Spawning this number of render threads.",
                cores
            );

            // Create a progress bar
            let mut progress =
                progress_bar::progress_bar::ProgressBar::new(my_image.height() as usize);
            progress.set_action(
                "Raytracing",
                progress_bar::color::Color::Blue,
                progress_bar::color::Style::Bold,
            );
            // Create a threadpool for rendering the image
            let mut pool = threadpool::ThreadPool::new(cores);
            // Number of active Jobs
            let mut active_jobs = 0;
            // Iterate over the image
            for (u, v, _) in my_image.enumerate_pixels() {
                // Collect Rays
                let mut rays = Vec::new();
                for _ in 0..ANTI_ALIASING {
                    // Get a ray to the pixel from the cam
                    rays.push(cam.get_ray(u, v).unwrap());
                }
                // Send the rays to the threadpool to calculate the color
                let world_clone = world.clone();
                let tx_color_clone = tx_color.clone();
                active_jobs += 1;
                pool.execute(move || {
                    // Save the Colors in its own vector
                    let mut color = Color::black();
                    for ray in rays {
                        color = color + world_clone.get_ray_color(ray, 0);
                    }
                    tx_color_clone
                        .send((u as usize, v as usize, color))
                        .unwrap();
                });

                // Check if enough jobs are sceduled to keep all threads busy
                while active_jobs == cores * 4 {
                    // Collect Results up to this point
                    while let Ok((x, y, color)) = rx_color.try_recv() {
                        active_jobs -= 1;
                        colors[x][y] = colors[x][y] + color;
                    }
                    std::thread::yield_now();
                }

                // Inc progress counter
                if u == 0 {
                    progress.inc();
                }
            }
        }
        // Collect results
        while let Ok((x, y, color)) = rx_color.try_recv() {
            colors[x][y] = colors[x][y] + color;
        }

        for (u, v, pixel) in my_image.enumerate_pixels_mut() {
            // Correct for anti aliasing
            let color = colors[u as usize][v as usize] / ANTI_ALIASING as f64;
            // Do some gamma corrections
            pixel.0[0] = (color.r().sqrt() * 255.9999) as u8;
            pixel.0[1] = (color.g().sqrt() * 255.9999) as u8;
            pixel.0[2] = (color.b().sqrt() * 255.9999) as u8;
        }
        println!();
    }
    println!("Saving image ...");
    my_image.save("test.png").unwrap();
    let end_time = std::time::Instant::now();
    println!("Time to calculate: {}s", (end_time - start_time).as_secs());
}
