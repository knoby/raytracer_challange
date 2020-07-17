//! Objects used to describe a scene for the ray tracer.

use crate::color::Color;
use crate::geometry::{Direction, Location};

const RAY_DEPTH_LIMIG: u32 = 50;

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    /// Create a sample world with a small and a very large sphere
    pub fn sample_world() -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(objects::Sphere {
            origin: Location::new(1.0, 0.0, 0.0),
            radius: 0.5,
        }));
        objects.push(Box::new(objects::Sphere {
            origin: Location::new(1.0, 0.0, -100.5),
            radius: 100.0,
        }));
        Self { objects }
    }

    /// Function returns the color of a given ray
    pub fn get_ray_color(&self, ray: Ray, depth: u32) -> Color {
        // If bouncing to much return black
        if depth >= RAY_DEPTH_LIMIG {
            return Color::black();
        }

        // Check if the ray is hitting something
        if let (_hit_distance, Some(hit_normal)) = self.get_hit(ray) {
            // Return color based on the normal
            let hit_normal = hit_normal.as_slice();
            // Calculate color based on the normal of the hit
            Color::new(-hit_normal[1] / 2.0 + 0.5, 0.0, 0.0)
                + Color::new(0.0, hit_normal[2] / 2.0 + 0.5, 0.0)
                + Color::new(0.0, 0.0, -hit_normal[0] / 2.0 + 0.5)
        } else {
            // Did not hit an object.. Return some background color
            let t = ray.direction.norm().z() / 2.0 + 0.5;
            Color::white() * (1.0 - t) + Color::blue() * t
        }
    }

    /// Iterate over all objects and get the one with a hit and the smales distance value
    fn get_hit(&self, ray: Ray) -> (f64, Option<Direction>) {
        self.objects
            .iter()
            .fold((f64::MAX, None), |act_hit, object| {
                if let Some(hit) = object.get_hits(&ray) {
                    // Check distance
                    if hit.distance < act_hit.0 && hit.distance > 0.0 {
                        (hit.distance, Some(hit.normal))
                    } else {
                        act_hit
                    }
                } else {
                    act_hit
                }
            })
    }
}

#[derive(Debug, Clone, Copy)]
/// Desrcribes a ray
pub struct Ray {
    pub origin: Location,
    pub direction: Direction,
}

/// Holding information about a hit of a ray and an object.
#[derive(Debug, Copy, Clone)]
pub struct Hit {
    /// Distance of the hint in direction of theray
    pub distance: f64,
    /// Direction of the normal in the hitpoint
    pub normal: Direction,
}

/// All Objects that interact in some way with a ray must implment this Trait
pub trait Hittable {
    fn get_hits(&self, ray: &Ray) -> Option<Hit>;
}

pub mod objects {
    use super::{Hit, Hittable, Ray};
    use crate::geometry::*;

    pub struct Sphere {
        pub origin: Location,
        pub radius: f64,
    }

    impl Hittable for Sphere {
        /// Checks if a ray hits the sphere
        fn get_hits(&self, ray: &Ray) -> Option<Hit> {
            // Algorithm from https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection with the sphere transformt to origin

            // Get stuff together
            let ray_orig = ray.origin - self.origin;
            let ray_dir = ray.direction;
            let r = self.radius;

            // Calculate wether it is a hit or not
            let d_3 = -ray_dir.dot(ray_orig);
            let d_1 = d_3.powi(2);
            let d_2 = ray_orig.length().powi(2) - r.powi(2);
            let discriminant = d_1 - d_2;

            if discriminant > 0.0 {
                let discriminant_sqrt = discriminant.sqrt();
                // Two Hits --> Return the one with the lowest positive Vlaue
                // Calculate the hits
                // Safe to call sqrt because we checked the value under the root
                let one_div_radius = 1.0 / self.radius;
                let distance = d_3 - discriminant_sqrt;
                let normal = (ray_dir * distance + ray_orig) * one_div_radius;
                let hit_1 = Hit { distance, normal };
                let distance = d_3 + discriminant_sqrt;
                let normal = (ray_dir * distance + ray_orig) * one_div_radius;
                let hit_2 = Hit { distance, normal };

                match (hit_1.distance, hit_2.distance) {
                    (d1, d2) if d1 > 0.0 && d2 <= 0.0 => Some(hit_1),
                    (d1, d2) if d2 > 0.0 && d1 <= 0.0 => Some(hit_2),
                    (d1, d2) if d1 <= 0.0 && d2 <= 0.0 => None,
                    (d1, d2) if d1 > 0.0 && d2 > 0.0 && d1 > d2 => Some(hit_2),
                    (d1, d2) if d1 > 0.0 && d2 > 0.0 && d1 < d2 => Some(hit_1),
                    (d1, d2) if d1 > 0.0 && d2 > 0.0 => Some(hit_1), // Both are Equeal and greater zero so the two above do not match. Return simply hit1
                    (_, _) => {
                        dbg!(hit_1);
                        dbg!(hit_2);
                        panic!("Unable to dertermen Order of hit_1 and hit_2 in sphere")
                    }
                }
            } else if discriminant < -0.0 {
                // No Hits
                None
            } else {
                // Just one hit
                let distance = d_3;
                if distance > 0.0 {
                    let normal = (ray_dir * distance + ray_orig) / self.radius;
                    Some(Hit { distance, normal })
                } else {
                    None
                }
            }
        }
    }
}
