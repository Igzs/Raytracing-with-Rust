use ray_tracing::{vec3,write_color};

fn main() {
    let width : i32= 256;
    let height : i32= 256;
    println!("P3\n{} {}\n255\n",width,height);

    for j in (0..height).rev() {
        eprintln!("Rows remaining {}",j);
        for i in 0..width {

        	let color = vec3::new(i as f64 / (width-1) as f64, j as f64 / (height-1) as f64, 0.25);
            write_color(color);


        }
    }
    eprintln!("Done!");
}


