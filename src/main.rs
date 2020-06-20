use std::ops;

#[derive(PartialEq,Debug)]
struct Vec3(f64, f64, f64);

impl Vec3 {
    fn unit() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }

    fn dot(&self, _rhs: &Vec3) -> f64 {
        self.0 * _rhs.0 +
        self.1 * _rhs.1 +
        self.2 * _rhs.2
    }

    fn len(&self) -> f64 {
        (self.0*self.0 +
         self.1*self.1 +
         self.2*self.2).sqrt()
    }

    fn cross(&self, _rhs: Vec3) -> Vec3 {
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
}

fn main() {
    // define extents
    let nx = 200;
    let ny = 100;

    // P3 indicates ASCII color output
    println!("P3");

    // Output the number of columns and rows
    println!("{} {}", nx, ny);

    // Output the maximum possible color value
    println!("255");

    // Output color information
    for j in (0..ny).rev() {
        for i in 0..nx {
            // compute color
            let col = Vec3(
                i as f64 / nx as f64,
                j as f64 / ny as f64,
                0.2
            );

            // compute intensity
            let intensity = col * 255.99;

            let ir = intensity.0 as i32;
            let ig = intensity.1 as i32;
            let ib = intensity.2 as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
