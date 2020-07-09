mod color;
mod direction;
//mod location;

fn main() {
    // Create a test image
    let mut my_image: image::RgbImage = image::RgbImage::new(1330, 768);

    // Get the size of the image
    let scale_x = my_image.width();
    let scale_y = my_image.height();

    {
        println!("Creating image...");
        let mut progress = progress_bar::progress_bar::ProgressBar::new(scale_y as usize);
        progress.set_action(
            "Raytracing",
            progress_bar::color::Color::Blue,
            progress_bar::color::Style::Bold,
        );

        for (x, y, pixel) in my_image.enumerate_pixels_mut() {
            pixel.0[0] = (255.0 * (scale_y - y) as f64 / scale_y as f64) as u8;
            pixel.0[1] = (255.0 * x as f64 / scale_x as f64) as u8;
            pixel.0[2] = (255.0 * y as f64 / scale_y as f64) as u8;
            if x == scale_x as u32 - 1 {
                progress.inc();
            }
        }
        println!();
    }
    println!("Saving image ...");
    my_image.save("test.png").unwrap();
}
