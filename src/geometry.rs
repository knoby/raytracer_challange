//! This module includes the implementation for geometry related things in the raytracer.
//! The most used ones are Location and Direction

/// Structure to hold information about a direction in the world.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Direction {
    data: [f64; 3],
}

impl Direction {
    /// Returns the x-component
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    /// Returns the y-component
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    /// Returns the z-component
    pub fn z(&self) -> f64 {
        self.data[2]
    }
    /// Returns the direction coordinates as a slice.
    pub fn as_slice(&self) -> [f64; 3] {
        self.data
    }

    /// Creates a new Direction with the given direction coefficients.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }

    /// Cross Product of two directions. Returns also a direction.
    pub fn cross(self, other: Self) -> Self {
        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.x() * other.z() - self.z() * other.x();
        let z = self.x() * other.y() - self.y() * other.x();

        Self::new(x, y, z)
    }

    /// Dot Product of two directions.
    pub fn dot(self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Returns the length of the direction vector.
    pub fn length(self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    /// Calculate the normalized form of the direction (Length == 1).
    pub fn norm(self) -> Self {
        self / self.length()
    }

    /// Inverts all components
    pub fn invert(self) -> Self {
        self * (-1.0)
    }
}

impl std::ops::Add for Direction {
    type Output = Direction;

    fn add(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
            ],
        }
    }
}

impl std::ops::Sub for Direction {
    type Output = Direction;

    fn sub(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
            ],
        }
    }
}

impl std::ops::Mul<f64> for Direction {
    type Output = Direction;

    fn mul(self, other: f64) -> Self {
        Self {
            data: [
                self.data[0] * other,
                self.data[1] * other,
                self.data[2] * other,
            ],
        }
    }
}

impl std::ops::Div<f64> for Direction {
    type Output = Direction;

    fn div(self, other: f64) -> Self {
        Self {
            data: [
                self.data[0] / other,
                self.data[1] / other,
                self.data[2] / other,
            ],
        }
    }
}

/// Struct holds information about the position in the world
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Location {
    data: [f64; 3],
}

impl Location {
    /// Returns the x-component
    pub fn x(&self) -> f64 {
        self.data[0]
    }

    /// Returns the x-component
    pub fn y(&self) -> f64 {
        self.data[1]
    }

    /// Returns the x-component
    pub fn z(&self) -> f64 {
        self.data[2]
    }

    /// Create new Location Object at the given coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }

    /// Creates a new Location at the origin (0.0, 0.0, 0.0).
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl std::ops::Sub for Location {
    type Output = Direction;

    fn sub(self, other: Self) -> Direction {
        Direction::new(
            self.data[0] - other.data[0],
            self.data[1] - other.data[1],
            self.data[2] - other.data[2],
        )
    }
}

impl std::ops::Add<Direction> for Location {
    type Output = Location;

    fn add(self, other: Direction) -> Self {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
            ],
        }
    }
}

impl std::ops::Sub<Direction> for Location {
    type Output = Location;

    fn sub(self, other: Direction) -> Self {
        Self {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
            ],
        }
    }
}
mod test {

    #[test]
    fn length() {
        let c1 = super::Direction::new(3.0, 4.0, 5.0);

        let l = c1.length();
        assert_eq!(l, 50.0_f64.sqrt());
    }

    #[test]
    fn dot() {
        let c1 = super::Direction::new(1.0, 0.0, 0.0);
        let c2 = super::Direction::new(0.0, 1.0, 0.0);

        let s = c1.dot(c2);
        assert_eq!(s, 0.0);
    }

    #[test]
    fn cross() {
        let c1 = super::Direction::new(1.0, 0.0, 0.0);
        let c2 = super::Direction::new(0.0, 1.0, 0.0);

        let c3 = c1.cross(c2);
        assert_eq!(c3.x(), 0.0);
        assert_eq!(c3.y(), 0.0);
        assert_eq!(c3.z(), 1.0);
    }

    #[test]
    fn add() {
        let c1 = super::Direction::new(0.0, 1.0, 2.0);
        let c2 = super::Direction::new(3.0, 4.0, 5.0);

        let c3 = c1 + c2;
        assert_eq!(c3.x(), 3.0);
        assert_eq!(c3.y(), 5.0);
        assert_eq!(c3.z(), 7.0);
    }

    #[test]
    fn sub() {
        let c1 = super::Direction::new(0.0, 1.0, 2.0);
        let c2 = super::Direction::new(3.0, 4.0, 5.0);

        let c3 = c1 - c2;
        assert_eq!(c3.x(), -3.0);
        assert_eq!(c3.y(), -3.0);
        assert_eq!(c3.z(), -3.0);
    }

    #[test]
    fn mul_f64() {
        let c1 = super::Direction::new(0.0, 1.0, 2.0);

        let c3 = c1 * 3.0;
        assert_eq!(c3.x(), 0.0);
        assert_eq!(c3.y(), 3.0);
        assert_eq!(c3.z(), 6.0);
    }

    #[test]
    fn div_f64() {
        let c1 = super::Direction::new(0.0, 1.0, 2.0);

        let c3 = c1 / 3.0;
        assert_eq!(c3.x(), 0.0);
        assert_eq!(c3.y(), 1.0 / 3.0);
        assert_eq!(c3.z(), 2.0 / 3.0);
    }
}
