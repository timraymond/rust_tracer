use std::ops;
use rand::Rng;

pub struct Camera {
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray{
            origin: self.origin,
            direction: self.lower_left + (self.horizontal * u) + (self.vertical * v) - self.origin,
        }
    }
}

pub trait Solid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct SolidGroup<T: Solid> {
    pub solids: Vec<T>,
}

impl<T: Solid> Solid for SolidGroup<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut rec = None;

        for solid in self.solids.iter() {
            if let Some(r) = solid.hit(r, t_min, closest) {
                closest = r.t;
                rec = Some(r);
            }
        }
        return rec;
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Solid for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius*self.radius;

        let discriminant = b*b - a*c;

        if discriminant > 0.0 {
            let near = (-b - (b*b-a*c).sqrt()) / a;
            if near < t_max && near > t_min {
                let p = r.point_at_param(near);
                return Some(HitRecord{
                    t: near,
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            }

            let far = (-b - (b*b-a*c).sqrt()) / a;

            if far < t_max && far > t_min {
                let p = r.point_at_param(far);

                return Some(HitRecord{
                    t: far,
                    p: p,
                    normal: (p - self.center) / self.radius,
                })
            }
        }

        return None;
    }
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_param(&self, magnitude: f64) -> Vec3 {
        self.origin + self.direction * magnitude
    }
}

#[derive(PartialEq,Debug,Copy,Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn unit() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn random(lower: f64, upper: f64) -> Vec3 {
        Vec3(
            rand::thread_rng().gen_range(lower, upper),
            rand::thread_rng().gen_range(lower, upper),
            rand::thread_rng().gen_range(lower, upper),
        )
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
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        (self.0*self.0 +
         self.1*self.1 +
         self.2*self.2)
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

