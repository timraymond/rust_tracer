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
    let lookat = Vec3(0.0, 0.0, -1.0);
    let lookfrom = Vec3(3.0, 3.0, 2.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).len();
    let aperature = 2.0;
    let aspect_ratio = nx as f64 / ny as f64;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperature,
        dist_to_focus,
    );

    // create geometry
    
    let sg = SolidGroup{
        solids: vec![
            Sphere{
                center: Vec3(0.0, 0.0, -1.0),
                radius: 0.5,
                material: &Lambertian{
                    albedo: Vec3(0.1, 0.2, 0.5),
                },
            },
            Sphere{
                center: Vec3(0.0, -100.5, -1.0),
                radius: 100.0,
                material: &Lambertian{
                    albedo: Vec3(0.8, 0.8, 0.0),
                },
            },
            Sphere{
                center: Vec3(1.0, 0.0, -1.0),
                radius: 0.5,
                material: &Metal{
                    albedo: Vec3(0.8, 0.6, 0.2),
                    fuzz: 0.0,
                },
            },
            Sphere{
                center: Vec3(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: &Dielectric{
                    ref_idx: 1.5,
                },
            },
            Sphere{
                center: Vec3(-1.0, 0.0, -1.0),
                radius: -0.45,
                material: &Dielectric{
                    ref_idx: 1.5,
                },
            },
            ],
        };

    let mut rng = rand::thread_rng();

    // Output color information
    for j in (0..ny).rev() {
        for i in 0..nx {

            let mut col = Vec3(0.0, 0.0, 0.0);

            for _ in 0..ns {
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
        Some(rec) => {
            let (atten, next) = rec.material.scatter(r, &rec.normal, &rec.p);
            color(&next, s, depth-1) * atten
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
