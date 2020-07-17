#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Color {
    data: [f64; 3],
}

impl Color {
    pub fn r(&self) -> f64 {
        self.data[0]
    }
    pub fn g(&self) -> f64 {
        self.data[1]
    }
    pub fn b(&self) -> f64 {
        self.data[2]
    }

    pub fn set_r(&mut self, r: f64) {
        self.data[0] = r;
    }

    pub fn set_g(&mut self, g: f64) {
        self.data[1] = g;
    }

    pub fn set_b(&mut self, b: f64) {
        self.data[2] = b;
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { data: [r, g, b] }
    }

    pub fn white() -> Self {
        Self {
            data: [1.0, 1.0, 1.0],
        }
    }

    pub fn black() -> Self {
        Self {
            data: [0.0, 0.0, 0.0],
        }
    }

    pub fn red() -> Self {
        Self {
            data: [1.0, 0.0, 0.0],
        }
    }

    pub fn green() -> Self {
        Self {
            data: [0.0, 1.0, 0.0],
        }
    }

    pub fn blue() -> Self {
        Self {
            data: [0.0, 0.0, 1.0],
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

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

impl std::ops::Sub for Color {
    type Output = Color;

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

impl std::ops::Mul<f64> for Color {
    type Output = Color;

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

impl std::ops::Div<f64> for Color {
    type Output = Color;

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

mod test {

    #[test]
    fn add() {
        let c1 = super::Color::new(0.0, 1.0, 2.0);
        let c2 = super::Color::new(3.0, 4.0, 5.0);

        let c3 = c1 + c2;
        assert_eq!(c3.r(), 3.0);
        assert_eq!(c3.g(), 5.0);
        assert_eq!(c3.b(), 7.0);
    }

    #[test]
    fn sub() {
        let c1 = super::Color::new(0.0, 1.0, 2.0);
        let c2 = super::Color::new(3.0, 4.0, 5.0);

        let c3 = c1 - c2;
        assert_eq!(c3.r(), -3.0);
        assert_eq!(c3.g(), -3.0);
        assert_eq!(c3.b(), -3.0);
    }

    #[test]
    fn mul_f64() {
        let c1 = super::Color::new(0.0, 1.0, 2.0);

        let c3 = c1 * 3.0;
        assert_eq!(c3.r(), 0.0);
        assert_eq!(c3.g(), 3.0);
        assert_eq!(c3.b(), 6.0);
    }

    #[test]
    fn div_f64() {
        let c1 = super::Color::new(0.0, 1.0, 2.0);

        let c3 = c1 / 3.0;
        assert_eq!(c3.r(), 0.0);
        assert_eq!(c3.g(), 1.0 / 3.0);
        assert_eq!(c3.b(), 2.0 / 3.0);
    }
}
