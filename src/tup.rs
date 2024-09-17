use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tup(pub f32, pub f32, pub f32);

impl Tup {
    pub fn norm(self) -> Self {
        self * (1.0 / (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt())
    }

    pub fn dot(self, rhs: Tup) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Tup) -> Tup {
        Tup(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl ops::Add<Tup> for Tup {
    type Output = Tup;

    fn add(self, rhs: Tup) -> Self::Output {
        Tup(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Tup> for Tup {
    type Output = Tup;

    fn sub(self, rhs: Tup) -> Self::Output {
        Tup(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<Tup> for Tup {
    type Output = Tup;

    fn mul(self, rhs: Tup) -> Self::Output {
        Tup(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<f32> for Tup {
    type Output = Tup;

    fn mul(self, rhs: f32) -> Self::Output {
        Tup(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_vec() {
        let v = Tup(1.0, 2.0, 3.0);

        assert_eq!(v.0, 1.0);
        assert_eq!(v.1, 2.0);
        assert_eq!(v.2, 3.0);
    }

    #[test]
    fn vec_add() {
        let v1 = Tup(1.0, 2.0, 3.0);
        let v2 = Tup(2.0, 3.0, 4.0);
        let v3 = v1 + v2;
        assert_eq!(v3.0, 3.0);
        assert_eq!(v3.1, 5.0);
        assert_eq!(v3.2, 7.0);
    }

    #[test]
    fn vec_sub() {
        let v1 = Tup(3.0, 2.0, 1.0);
        let v2 = Tup(1.0, 2.0, 3.0);
        let v3 = v1 - v2;
        assert_eq!(v3.0, 2.0);
        assert_eq!(v3.1, 0.0);
        assert_eq!(v3.2, -2.0);
    }

    #[test]
    fn vec_mul_vec() {
        let v1 = Tup(3.0, 2.0, 1.0);
        let v2 = Tup(1.0, 2.0, 3.0);
        let v3 = v1 * v2;
        assert_eq!(v3.0, 3.0);
        assert_eq!(v3.1, 4.0);
        assert_eq!(v3.2, 3.0);
    }

    #[test]
    fn vec_mul_f32() {
        let v1 = Tup(1.0, 2.0, 3.0);
        let a: f32 = 3.0;

        let v3 = v1 * a;
        assert_eq!(v3.0, 3.0);
        assert_eq!(v3.1, 6.0);
        assert_eq!(v3.2, 9.0);
    }

    #[test]
    fn vec_dot() {
        let v1 = Tup(1.0, 2.0, 3.0);
        let v2 = Tup(2.0, 3.0, 4.0);
        let a: f32 = v1.dot(v2);
        assert_eq!(a, 20.0);
    }

    #[test]
    fn vec_cross() {
        let v1 = Tup(1.0, 2.0, 3.0);
        let v2 = Tup(2.0, 3.0, 4.0);
        let a = v1.cross(v2);
        assert_eq!(a.0, -1.0);
        assert_eq!(a.1, 2.0);
        assert_eq!(a.2, -1.0);
    }
}
