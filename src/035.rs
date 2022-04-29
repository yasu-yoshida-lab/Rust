fn main() 
{ 
	let mut vec: Vec<i32> = vec![1, 2, 3];
	let moved = vec;
	vec = vec![4, 5, 6];
	println!("{:?}", moved);
	println!("{:?}", vec);
}
