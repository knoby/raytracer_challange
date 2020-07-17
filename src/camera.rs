//! In this module the camera ist implemented. The main goal is to provide an iterator that returns all the rays from the camera

use crate::geometry::Direction;
use crate::geometry::Location;
use crate::scene::Ray;
use rand::prelude::*;

// Definitins only for calculation
/// Height of the Viewport. Is used for some Calculations internaly.
/// Can't be changed, and is not needed.
const VIEWPORT_HEIGHT: f64 = 2.0;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    /// Location of the camera
    origin: Location,
    /// Direction in whicht the camera points
    direction: Direction,
    /// Aspect Ration of the final image (Width/Height). For Exampel: 4/3, 16/9, 1/1
    aspect_ratio: f64,
    /// Dimensions of the viewport in pixels (Width, Height)
    viewport_size: (u32, u32),
    /// Focal Length (Distance of the viewport from the origin)
    focal_length: f64,
    /// Position of the upper Left corener of the Viewport
    viewport_top_left: Location,
    /// Horizontal Direction of the viewport
    viewport_horizontal: Direction,
    /// Vertical Direction of the viewport
    viewport_vertical: Direction,
    /// Random number generator for anti alysing
    rng: rand::rngs::ThreadRng,
}

impl Camera {
    /// Create a new Camera Object. After Createion the rays can be requested by calling the next() method.
    pub fn new(
        origin: Location,
        direction: Direction,
        width: u32,
        height: u32,
        focal_length: f64,
    ) -> Self {
        // Calculate data from the inputs
        let aspect_ratio = width as f64 / height as f64;
        // Calculate the horizontal direction of the viewport
        let vert_rotation = direction.y().atan2(direction.x());
        let horizontal_angle = vert_rotation - std::f64::consts::FRAC_PI_2;
        let viewport_horizontal =
            Direction::new(horizontal_angle.cos(), horizontal_angle.sin(), 0.0);
        // The Vertical Direction of is the cross product of the origin and the horizontal direction
        let viewport_vertical = viewport_horizontal.cross(direction).norm();
        // From this values we can calculate the top left corner of the viewport. The Imaginaray viewport is 2 unit heigth and 2*aspect_ratio width
        let viewport_top_left = Location::origin()
            + direction.norm() * focal_length
            + viewport_vertical * 0.5 * VIEWPORT_HEIGHT
            - viewport_horizontal * 0.5 * VIEWPORT_HEIGHT * aspect_ratio;

        // Create the Struct
        Camera {
            origin,
            direction,
            aspect_ratio,
            focal_length,
            viewport_horizontal,
            viewport_vertical,
            viewport_top_left,
            viewport_size: (width, height),
            rng: thread_rng(),
        }
    }

    pub fn get_ray(&mut self, u: u32, v: u32) -> Result<Ray, String> {
        // Create random offset in pixel
        let u_r: f64 = self.rng.gen();
        let v_r: f64 = self.rng.gen();
        // Check if the target pixel is within the viewport
        if u < self.viewport_size.0 && v < self.viewport_size.1 {
            let viewport_target = self.viewport_top_left
                + self.viewport_horizontal
                    * (self.aspect_ratio / self.viewport_size.0 as f64
                        * (u as f64 + u_r)
                        * VIEWPORT_HEIGHT)
                - self.viewport_vertical
                    * (1.0 / self.viewport_size.1 as f64 * (v as f64 + v_r) * VIEWPORT_HEIGHT);

            // Construct the ray
            let ray = Ray {
                direction: (viewport_target - Location::origin()).norm(),
                origin: self.origin,
            };
            Ok(ray)
        } else {
            Err("Pixel out of Viewport".into())
        }
    }
}
