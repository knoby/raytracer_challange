#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Direction {
    data: [f64; 3],
}

impl Direction {
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

impl std::ops::Add for Direction {
    type Output = Direction;

    fn add(self, other: Self) -> Self {
        let mut data = [0.0; 3];

        for ((s, o), d) in self.data.iter().zip(other.data.iter()).zip(data.iter_mut()) {
            *d = s + o;
        }

        Self { data }
    }
}

impl std::ops::Sub for Direction {
    type Output = Direction;

    fn sub(self, other: Self) -> Self {
        let mut data = [0.0; 3];

        for ((s, o), d) in self.data.iter().zip(other.data.iter()).zip(data.iter_mut()) {
            *d = s - o;
        }

        Self { data }
    }
}

impl std::ops::Mul<f64> for Direction {
    type Output = Direction;

    fn mul(self, other: f64) -> Self {
        let mut data = [0.0; 3];

        for (s, d) in self.data.iter().zip(data.iter_mut()) {
            *d = s * other;
        }

        Self { data }
    }
}

impl std::ops::Div<f64> for Direction {
    type Output = Direction;

    fn div(self, other: f64) -> Self {
        let mut data = [0.0; 3];

        for (s, d) in self.data.iter().zip(data.iter_mut()) {
            *d = s / other;
        }

        Self { data }
    }
}

mod test {

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
