mod vec3;
mod geometry;
mod material;

use vec3::*;

use material::{
    Lambertian,
    Dielectric,
    Metal,
    Material
};

use geometry::{
    Solid,
    SolidGroup,
    Sphere
};

use rand::Rng;

fn main() {
    // define extents
    let nx = 4096;
    let ny = 2160;
    let ns = 100;

    let max_depth = 50;

    // P3 indicates ASCII color output
    println!("P3");

    // Output the number of columns and rows
    println!("{} {}", nx, ny);

    // Output the maximum possible color value
    println!("255");

    // Setup vectors for ray tracing
    let lookat = Vec3(0.0, 0.0, 0.0);
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
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
    let sg = random_scene();

    let mut rng = rand::thread_rng();

    // Output color information
    for j in (0..ny).rev() {
        eprintln!("Scanlines remaining: {}", j);
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
    eprintln!("Done!");
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

fn random_scene() -> SolidGroup<Sphere> {
    let mut scene = vec!(
        Sphere{
            center: Vec3(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(Lambertian{
                albedo: Vec3(0.5, 0.5, 0.5),
            }),
        },
    );

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let ra: f64 = rng.gen();
            let rb: f64 = rng.gen();

            let center = Vec3(
                a as f64 + ra * 0.9,
                0.2,
                b as f64 + rb * 0.9,
            );

            let radius = 0.2;

            if (center - Vec3(4.0, 0.2, 0.0)).len() > 0.9 {
                let material = random_mat();
                scene.push(Sphere{
                    center,
                    radius,
                    material: material,
                });
            }
        }
    }

    scene.push(Sphere{
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric{
            ref_idx: 1.5,
        }),
    });

    scene.push(Sphere{
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian{
            albedo: Vec3(0.4, 0.2, 0.1),
        }),
    });

    scene.push(Sphere{
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal{
            albedo: Vec3(0.7, 0.6, 0.0),
            fuzz: 0.0,
        }),
    });

    return SolidGroup{
        solids: scene,
    };
}

fn random_mat() -> Box<dyn Material> {
    let mut rng = rand::thread_rng();
    let choose_mat: f64 = rng.gen();

    if choose_mat < 0.8 {
        Box::new(Lambertian{
            albedo: Vec3(
                rng.gen(),
                rng.gen(),
                rng.gen(),
            ),
        })
    } else if choose_mat < 0.95 {
        Box::new(Metal{
            albedo: Vec3(
                rng.gen_range(0.5, 1.0),
                rng.gen_range(0.5, 1.0),
                rng.gen_range(0.5, 1.0),
            ),
            fuzz: rng.gen_range(0.0, 0.5),
        })
    } else {
        Box::new(Dielectric{
            ref_idx: 1.5,
        })
    }
}
