fn print_vector_capacity() {
	println!("\n[print vector capacity]");
	let mut v: Vec<i32> = Vec::with_capacity(3);
	println!("v.len()      = {}", v.len());
	println!("v.capacity() = {}", v.capacity());
	v.push(1);
	println!("v.len()      = {}", v.len());
	println!("v.capacity() = {}", v.capacity());
}

fn main() { 
	print_vector_capacity();
}
