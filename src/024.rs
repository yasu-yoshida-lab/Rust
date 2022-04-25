fn get_type<T>(_: T) -> &'static str {
	std::any::type_name::<T>()
}

fn print_vector_example() {
	println!("\n[print vector example]");
	println!("vec![0, 1, 2]         = {:?} {}", vec![0, 1, 2], get_type(vec![0, 1, 2]));
	println!("vec![0; 3]            = {:?} {}", vec![0; 3],    get_type(vec![0; 3]));
	let mut v: Vec<i32> = Vec::new();
	println!("Vec::new()            = {:?} {}", v, get_type(v.clone()));
	let mut v: Vec<i32> = Vec::with_capacity(3);
	println!("Vec::with_capacity(3) = {:?} {}", v, get_type(v.clone()));
	let v: Vec<i32> = (0..3).collect();
	println!("(0..3).collect()      = {:?} {}", v, get_type(v.clone()));
	println!("vec![vec![1; 2]; 3]   = {:?} {}", vec![vec![1; 2]; 3], get_type(vec![vec![1; 2]; 3]));
}

fn main() { 
	print_vector_example();
}
