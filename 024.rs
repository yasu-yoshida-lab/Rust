fn get_type<T>(_: T) -> &'static str { 
	std::any::type_name::<T>() 
}

fn print_vector_example() { 
	println!("[print vector example]");
	println!("vec![0, 1, 2] = {:?} {}", vec![0, 1, 2], get_type(vec![0, 1, 2]));
	println!("vec![0;, 3] = {:?} {}", vec![0; 3], get_type(vec![0; 3]));
	let mut x : Vec<i32> = Vec::new();
	println!("Vec::new() = {:?} {}", x , get_type(x.clone()));
	let mut y : Vec<i32> = Vec::with_capacity(3); 
	println!("Vec::with_capacity(3) = {:?} {}", y, get_type(y.clone()));
	let z : Vec<i32> = (0..3).collect();
	println!("(0..3).collect() = {:?} {}", z, get_type(z.clone()));
	println!("vec![vec![1; 2]; 3] = {:?} {}", vec![vec![1; 2]; 3], get_type(vec![vec![1; 2]; 3]));
}

fn main() {
	print_vector_example();
}
