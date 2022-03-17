use std::fmt;

pub struct vec3{
	x : f64,
	y : f64,
	z : f64
}

impl fmt::Display for vec3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value x: {}, value y: {}, value z: {})", self.x, self.y, self.z)

	}

}

impl vec3 {
	pub fn new(x : f64, y : f64, z : f64) -> vec3 {
		vec3 { x, y ,z}
	}

	pub fn add(mut self, vec: vec3) -> vec3 {
		self.x += vec.x;
		self.y += vec.y;
		self.z += vec.z;
		self
	}

	pub fn multiply_constant(mut self, constant : u64) -> vec3 {
		self.x *= constant as f64;
		self.y *= constant as f64;
		self.z *= constant as f64;
		self
	}

	pub fn divide_by_constant(self, constant: u64) -> vec3 {

		self.multiply_constant(1/constant)
	}



}

pub fn write_color(color : vec3) {
	let ir = (255.999 * color.x) as u8;
    let ig = (255.999 * color.y) as u8;
    let ib = (255.999 * color.z) as u8;

    println!("{} {} {}\n",ir,ig,ib);
}

