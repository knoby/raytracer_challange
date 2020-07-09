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

    pub fn as_slice(&self) -> [f64; 3] {
        self.data
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn cross(self, other: Self) -> Self {
        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.x() * other.z() - self.z() * other.x();
        let z = self.x() * other.y() - self.y() * other.x();

        Self::new(x, y, z)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn length(self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn norm(self) -> Self {
        self / self.length()
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
