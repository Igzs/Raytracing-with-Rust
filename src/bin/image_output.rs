fn main() {
    let width : i32= 256;
    let height : i32= 256;
    println!("P3\n{} {}\n255\n",width,height);

    for j in (0..height).rev() {
        eprintln!("Rows remaining {}",j);
        for i in 0..width {
            let r = i as f64 / (width-1) as f64;
            let g = j as f64 / (height-1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{} {} {}\n",ir,ig,ib);
        }
    }
    eprintln!("Done!");
}


