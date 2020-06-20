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
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2 as f32;

            // compute intensity
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
