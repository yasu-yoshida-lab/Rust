fn second_any_i32<T>(tuple: (T, i32)) -> i32 
{ 
	return tuple.1;
}

fn second<T, U>(tuple: (T, U)) -> U 
{
	return tuple.1;
}

fn main() 
{ 
	let t: (f64, i32) = (3.00, 5);
	println!("{}", second_any_i32::<f64>(t));
	let t: (f32, i32) = (3.00, 5);
	println!("{}", second_any_i32::<f32>(t));
	let t: (bool, i32) = (true, 5);
	println!("{}", second_any_i32::<bool>(t));

	let t: (bool, bool) = (true, false);
	println!("{}", second::<bool, bool>(t));
	let t: (i32, f64) = (5, 6.43);
	println!("{}", second::<i32, f64>(t));

	let t: (i32, char) = (5, 'A');
	println!("{}", second::<i32, _>(t));
}
