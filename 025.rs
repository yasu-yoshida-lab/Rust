fn get_type<T>(_: T) -> &'static str { 
	std::any::type_name::<T>() 
}

fn print_vector_capacity() { 
	println!("[print vector capacity]");
	let mut v : Vec<i32> = Vec::with_capacity(3);
	println!("v.len() = {}", v.len());
	println!("v.capacity = {}", v.capacity());
	v.push(1);
	println!("v.len() = {}", v.len());
	println!("v.capacity = {}", v.capacity());
}

fn main() { 
	print_vector_capacity();
}
