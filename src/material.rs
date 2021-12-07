use super::vec3::{Vec3,Ray};
use rand::Rng;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let a: f64 = rng.gen_range(0.0, 2.0*std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - (z*z)).sqrt();

    Vec3(r*a.cos(), r*a.sin(), z)
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

pub struct Dielectric {
    pub ref_idx: f64,
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

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0-ref_idx) / (1.0 + ref_idx);
    let r1 = r0*r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
}

fn refract(inv: Vec3, norm: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-inv).dot(&norm);
    
    let r_out_parallel = (inv + norm * cos_theta) * etai_over_etat;
    let r_out_perp = norm * (-(1.0 - r_out_parallel.len_squared()).sqrt());

    return r_out_parallel + r_out_perp;
}

