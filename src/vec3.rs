use std::ops;

pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_param(self, magnitude: f64) -> Vec3 {
        self.origin + self.direction * magnitude
    }
}

#[derive(PartialEq,Debug,Copy,Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn unit() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn to_unit(&self) -> Vec3 {
        let k = 1.0 / self.len();
        Vec3(self.0 * k,
             self.1 * k,
             self.2 * k)
    }

    pub fn dot(&self, _rhs: &Vec3) -> f64 {
        self.0 * _rhs.0 +
        self.1 * _rhs.1 +
        self.2 * _rhs.2
    }

    pub fn len(&self) -> f64 {
        (self.0*self.0 +
         self.1*self.1 +
         self.2*self.2).sqrt()
    }

    pub fn cross(&self, _rhs: Vec3) -> Vec3 {
        Vec3(self.1*_rhs.2 - self.2*_rhs.1,
             -(self.0*_rhs.2 - self.2*_rhs.0),
             self.0*_rhs.1 - self.1*_rhs.0)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3(self.0 + _rhs.0,
             self.1 + _rhs.1,
             self.2 + _rhs.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3(self.0 - _rhs.0,
             self.1 - _rhs.1,
             self.2 - _rhs.2)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3(self.0 * _rhs.0,
             self.1 * _rhs.1,
             self.2 * _rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3(self.0 * _rhs,
             self.1 * _rhs,
             self.2 * _rhs)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3(self.0 / _rhs.0,
             self.1 / _rhs.1,
             self.2 / _rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3(self.0 / _rhs,
             self.1 / _rhs,
             self.2 / _rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_add() {
        let a = Vec3(1.0, 1.0, 1.0);
        let b = Vec3(1.0, 1.0, 1.0);
        let c = a + b;

        assert_eq!(2.0, c.0);
        assert_eq!(2.0, c.1);
        assert_eq!(2.0, c.2);
    }

    #[test]
    fn test_sub() {
        let a = Vec3(2.0, 2.0, 2.0);
        let b = Vec3(2.0, 2.0, 2.0);
        let c = a - b;

        assert_eq!(0.0, c.0);
        assert_eq!(0.0, c.1);
        assert_eq!(0.0, c.2);
    }

    #[test]
    fn test_mul_vec() {
        let a = Vec3(2.0, 2.0, 2.0);
        let b = Vec3(2.0, 2.0, 2.0);
        let c = a * b;

        assert_eq!(4.0, c.0);
        assert_eq!(4.0, c.1);
        assert_eq!(4.0, c.2);
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vec3(2.0, 2.0, 2.0);
        let c = a * 2.0;

        assert_eq!(4.0, c.0);
        assert_eq!(4.0, c.1);
        assert_eq!(4.0, c.2);
    }

    #[test]
    fn test_div_vec() {
        let a = Vec3(2.0, 2.0, 2.0);
        let b = Vec3(2.0, 2.0, 2.0);
        let c = a / b;

        assert_eq!(1.0, c.0);
        assert_eq!(1.0, c.1);
        assert_eq!(1.0, c.2);
    }

    #[test]
    fn test_div_scalar() {
        let a = Vec3(2.0, 2.0, 2.0);
        let c = a / 2.0;

        assert_eq!(1.0, c.0);
        assert_eq!(1.0, c.1);
        assert_eq!(1.0, c.2);
    }

    #[test]
    fn test_dot() {
        let a = Vec3(2.0, 2.0, 2.0);
        let b = Vec3(2.0, 2.0, 2.0);

        assert_eq!(12.0, a.dot(&b));
    }

    #[test]
    fn test_unit() {
        let a = Vec3::unit();

        assert_eq!(1.0, a.0);
        assert_eq!(1.0, a.1);
        assert_eq!(1.0, a.2);
    }

    #[test]
    fn test_len() {
        let a = Vec3::unit();

        assert_eq!(1732, (a.len() * 1000.0) as i32);
    }

    #[test]
    fn test_cross() {
        let a = Vec3(3.0, 2.0, -2.0);
        let b = Vec3(1.0, 0.0, -5.0);

        let exp = Vec3(-10.0, 13.0, -2.0);

        assert_eq!(exp, a.cross(b));
    }

    use super::Ray;

    #[test]
    fn test_ray() {
        let r = Ray{
            origin: Vec3(0.0, 0.0, 0.0),
            direction: Vec3(1.0, 0.0, 0.0),
        };

        let exp = Vec3(2.0, 0.0, 0.0);

        assert_eq!(exp, r.point_at_param(2.0))
    }
}

