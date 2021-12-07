use super::vec3::{Ray,Vec3};
use super::material::{Material};

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Solid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
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
                    material: self.material.as_ref(),
                    normal: (p - self.center) / self.radius,
                });
            }

            let far = (-b - (b*b-a*c).sqrt()) / a;

            if far < t_max && far > t_min {
                let p = r.point_at_param(far);

                return Some(HitRecord{
                    t: far,
                    p: p,
                    material: self.material.as_ref(),
                    normal: (p - self.center) / self.radius,
                });
            }
        }

        return None;
    }
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

