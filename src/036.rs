fn main() 
{ 
	let vec: Vec<i32> = vec![1, 2, 3];
	let ref_vec = &vec;
	println!("{:?}", vec);
	println!("{:?}", ref_vec);
}
