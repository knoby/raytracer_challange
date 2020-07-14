//! Objects used to describe a scene for the ray tracer.

use crate::geometry::{Direction, Location};

#[derive(Debug, Clone, Copy)]
/// Desrcribes a ray
pub struct Ray {
    pub origin: Location,
    pub direction: Direction,
}

/// Holding information about a hit of a ray and an object.
pub struct Hit {}

/// All Objects that interact in some way with a ray must implment this Trait
pub trait Hittable {
    fn get_hits(&self, ray: &Ray) -> Option<Vec<Hit>>;
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
        fn get_hits(&self, ray: &Ray) -> Option<Vec<Hit>> {
            // Algorithm from https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection with the sphere transformt to origin

            // Get stuff together
            let ray_orig = ray.origin - self.origin;
            let ray_dir = ray.direction.norm();
            let r = self.radius;

            // Calculate wether it is a hit or not
            let d_1 = ray_dir.dot(ray_orig).powi(2);
            let d_2 = ray_orig.length().powi(2) - r.powi(2);
            let discriminant = d_1 - d_2;

            if discriminant > 0.0 {
                let d_3 = -ray_dir.dot(ray_orig);
                Some(vec![Hit {}, Hit {}])
            } else if discriminant < -0.0 {
                None
            } else {
                Some(vec![Hit {}])
            }
        }
    }
}
