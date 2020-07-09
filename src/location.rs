use super::direction::Direction;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Location {
    data: [f64; 3],
}

impl Location {
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }
}

impl std::ops::Sub for Location {
    type Output = Direction;

    fn sub(self, other: Self) -> Direction {
        let mut data = [0.0; 3];

        for ((s, o), d) in self.data.iter().zip(other.data.iter()).zip(data.iter_mut()) {
            *d = s - o;
        }

        Direction::new(data[0], data[1], data[2])
    }
}

impl std::ops::Add<Direction> for Location {
    type Output = Location;

    fn add(self, other: Direction) -> Self {
        let mut data = [0.0; 3];

        for ((s, o), d) in self
            .data
            .iter()
            .zip(other.as_slice().iter())
            .zip(data.iter_mut())
        {
            *d = s + o;
        }

        Self { data }
    }
}

impl std::ops::Sub<Direction> for Location {
    type Output = Location;

    fn sub(self, other: Direction) -> Self {
        let mut data = [0.0; 3];

        for ((s, o), d) in self
            .data
            .iter()
            .zip(other.as_slice().iter())
            .zip(data.iter_mut())
        {
            *d = s - o;
        }

        Self { data }
    }
}

mod test {

    #[test]
    fn sub() {
        let c1 = super::Location::new(0.0, 1.0, 2.0);
        let c2 = super::Location::new(3.0, 4.0, 5.0);

        let c3 = c1 - c2;
        assert_eq!(c3.x(), -3.0);
        assert_eq!(c3.y(), -3.0);
        assert_eq!(c3.z(), -3.0);
    }

    #[test]
    fn add_direction() {
        let c1 = super::Location::new(0.0, 1.0, 2.0);
        let c2 = super::Direction::new(3.0, 4.0, 5.0);

        let c3 = c1 + c2;
        assert_eq!(c3.x(), 3.0);
        assert_eq!(c3.y(), 5.0);
        assert_eq!(c3.z(), 7.0);
    }

    #[test]
    fn sub_direction() {
        let c1 = super::Location::new(0.0, 1.0, 2.0);
        let c2 = super::Direction::new(3.0, 4.0, 5.0);

        let c3 = c1 - c2;
        assert_eq!(c3.x(), -3.0);
        assert_eq!(c3.y(), -3.0);
        assert_eq!(c3.z(), -3.0);
    }
}
