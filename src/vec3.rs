use std::ops;
use rand::Rng;

pub struct Camera {
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Vec3(0.0, 0.0, 0.0);
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);

        Camera{
            origin,
            horizontal,
            vertical,
            lower_left: origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length),
        }
    }
}

pub trait Material {
    fn scatter(&self, r: &Ray, normal: &Vec3, origin: &Vec3) -> (Vec3, Ray);
}

/// Lambertian describes a perfectly diffuse material of a certain color
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, normal: &Vec3, origin: &Vec3) -> (Vec3, Ray) {
        let scattered = Ray{
            origin: *origin,
            direction: random_in_unit_sphere() + *normal,
        };

        (self.albedo, scattered)
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, normal: &Vec3, origin: &Vec3) -> (Vec3, Ray) {
        let scattered = Ray{
            origin: *origin,
            direction: reflect(r.direction.to_unit(), *normal) + (random_in_unit_sphere() * self.fuzz),
        };

        (self.albedo, scattered)
    }
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    return incident - normal * (incident.dot(&normal) * 2.0);
}

fn refract(inv: Vec3, norm: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-inv).dot(&norm);
    
    let r_out_parallel = (inv + norm * cos_theta) * etai_over_etat;
    let r_out_perp = norm * (-(1.0 - r_out_parallel.len_squared()).sqrt());

    return r_out_parallel + r_out_perp;
}

pub struct Dielectric {
    pub ref_idx: f64,
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0-ref_idx) / (1.0 + ref_idx);
    let r1 = r0*r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, normal: &Vec3, origin: &Vec3) -> (Vec3, Ray) {
        let front_face = r.direction.dot(normal) < 0.0;

        let norm = if front_face {
            *normal
        } else {
            -(*normal)
        };

        let unit_direction = r.direction.to_unit();

        let cos_theta = (-unit_direction).dot(&norm).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let attenuation = Vec3(1.0, 1.0, 1.0);

        let etai_over_etat = if front_face {
            self.ref_idx.recip()
        } else {
            self.ref_idx
        };

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        let should_reflect: f64 = rand::thread_rng().gen();


        if etai_over_etat * sin_theta > 1.0 {
            // total internal reflection
            let scattered = Ray{
                origin: *origin,
                direction: reflect(unit_direction, norm),
            };
            return (attenuation, scattered);
        }

        if should_reflect < reflect_prob {
            // total internal reflection
            let scattered = Ray{
                origin: *origin,
                direction: reflect(unit_direction, norm),
            };
            return (attenuation, scattered);
        }

        let refracted = refract(unit_direction, norm, etai_over_etat);

        let scattered = Ray{
            origin: *origin,
            direction: refracted,
        };

        (attenuation, scattered)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let a: f64 = rng.gen_range(0.0, 2.0*std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - (z*z)).sqrt();

    Vec3(r*a.cos(), r*a.sin(), z)
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

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl <'a> Solid for Sphere<'a> {
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
                    material: self.material,
                    normal: (p - self.center) / self.radius,
                });
            }

            let far = (-b - (b*b-a*c).sqrt()) / a;

            if far < t_max && far > t_min {
                let p = r.point_at_param(far);

                return Some(HitRecord{
                    t: far,
                    p: p,
                    material: self.material,
                    normal: (p - self.center) / self.radius,
                });
            }
        }

        return None;
    }
}

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * -1.0
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

