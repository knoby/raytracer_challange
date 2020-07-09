use crate::direction::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Add<Direction> for Location {
    type Output = Location;

    fn add(self, other: Direction) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Direction> for Location {
    type Output = Location;

    fn sub(self, other: Direction) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
