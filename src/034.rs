fn main() 
{
	let vec: Vec<i32> = vec![1, 2, 3];
	let moved = vec;
	println!("moved : {:?}", moved);
	/* Error */
	// println!("vec : {:?}", vec);
}
