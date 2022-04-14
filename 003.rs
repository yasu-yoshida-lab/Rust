use std::mem;

fn main() { 
	let mut a = 0;
	let mut b = 1;
	println!("a = {}, b = {}", a, b);
	mem::swap(&mut a, &mut b);
	println!("a = {}, b = {}", a, b);
}
