mod vec3;

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
            let col = vec3::Vec3(
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
