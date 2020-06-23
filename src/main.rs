mod vec3;

use vec3::*;
use rand::Rng;

fn main() {
    // define extents
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let max_depth = 50;

    // P3 indicates ASCII color output
    println!("P3");

    // Output the number of columns and rows
    println!("{} {}", nx, ny);

    // Output the maximum possible color value
    println!("255");

    // Setup vectors for ray tracing
    let cam = Camera{
        lower_left : Vec3(-2.0 , -1.0 , -1.0),
        horizontal : Vec3(4.0  , 0.0  , 0.0),
        vertical   : Vec3(0.0  , 2.0  , 0.0),
        origin     : Vec3(0.0  , 0.0  , 0.0),
    };

    // create geometry
    
    let sg = SolidGroup{
        solids: vec![
            Sphere{
                center: Vec3(0.0, 0.0, -1.0),
                radius: 0.5,
            },
            Sphere{
                center: Vec3(0.0, -100.5, -1.0),
                radius: 100.0,
            },
            ],
        };

    let mut rng = rand::thread_rng();

    // Output color information
    for j in (0..ny).rev() {
        for i in 0..nx {

            let mut col = Vec3(0.0, 0.0, 0.0);

            for s in (0..ns) {
                // Compute pixel offsets as percentage in range 0.0 < u,v < 1.0
                let ru: f64 = rng.gen();
                let rv: f64 = rng.gen();

                let u = ((i as f64) + ru) / nx as f64;
                let v = ((j as f64) + rv) / ny as f64;

                let r = cam.get_ray(u, v);
                col = color(&r, &sg, max_depth) + col;
            }

            col = col / ns as f64;

            let ir = (col.0.sqrt() * 255.99) as i32;
            let ig = (col.1.sqrt() * 255.99) as i32;
            let ib = (col.2.sqrt() * 255.99) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &Ray, s: &dyn Solid, depth: isize) -> Vec3 {

    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0)
    }

    match s.hit(r, 0.001, std::f64::MAX) {
        Some(r) => {
            let target = r.p + r.normal + random_in_unit_sphere();

            let next = Ray{
                origin: r.p,
                direction: target - r.p,
            };

            color(&next, s, depth-1) * 0.5
        }
        
        None => {
            // extract a unit direction vector from the ray
            let Vec3(_, y, _) = r.direction.to_unit();

            // t is the "blueness". When t=1.0 we want blue, otherwise we want white. We compute t by the
            // "upness" and "downness"
            let t = 0.5 * (y + 1.0);
                (Vec3(1.0, 1.0, 1.0) * (1.0-t)) +
                 (Vec3(0.5, 0.7, 1.0) * t)
        }
    }

}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let a: f64 = rng.gen_range(0.0, 2.0*std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - (z*z)).sqrt();

    Vec3(r*a.cos(), r*a.sin(), z)
}
