fn main() 
{
	let val: i32 = sum(&[5, -1, 3, 4, -2]);
	println!("value : {}", val);
}

fn sum(slice: &[i32]) -> i32
{
	if slice.is_empty() { 
		return 0;
	}
	else { 
		return sum(&slice[1..]) + slice[0];
	}
}
