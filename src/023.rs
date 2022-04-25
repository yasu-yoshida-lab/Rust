fn get_type<T>(_: T) -> &'static str {
	std::any::type_name::<T>()
}

fn print_array_access() {
	println!("\n[print array access]");
	let a: [i32; 3] = [0, 1, 2];
	println!("a[0] = {}", a[0]);
	let i = 0;
	println!("i={}; a[i] = {}", i, a[i]);
	let b: [[i32; 3]; 1] = [a];
	println!("b[0][0] = {}", b[0][0]);
	let b = [[0;3]; 8];
	println!("b = {:?} {}", b, get_type(b));
}

fn main() {
	print_array_access();
}
