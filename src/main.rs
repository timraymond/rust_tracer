mod vec3;

use vec3::*;

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

    // Setup vectors for ray tracing
    let lower_left = Vec3(-2.0 , -1.0 , -1.0);
    let horizontal = Vec3(4.0  , 0.0  , 0.0);
    let vertical   = Vec3(0.0  , 2.0  , 0.0);
    let origin     = Vec3(0.0  , 0.0  , 0.0);

    // Output color information
    for j in (0..ny).rev() {
        for i in 0..nx {
            // Compute pixel offsets as percentage in range 0.0 < u,v < 1.0
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            // create ray to trace
            let r = Ray{
                origin: origin,
                direction: lower_left + (horizontal * u + vertical * v),
            };

            let col = color(r) * 255.99;

            let ir = col.0 as i32;
            let ig = col.1 as i32;
            let ib = col.2 as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: Ray) -> Vec3 {
    // extract a unit direction vector from the ray
    let Vec3(_, y, _) = r.direction.to_unit();

    // t is the "blueness". When t=1.0 we want blue, otherwise we want white. We compute t by the
    // "upness" and "downness"
    let t = 0.5 * (y + 1.0);
    (Vec3(1.0, 1.0, 1.0) * (1.0-t)) +
         (Vec3(0.5, 0.7, 1.0) * t)
}
