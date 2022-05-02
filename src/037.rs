fn main()
{ 
	let vec: Vec<Vec<i32>> = vec![vec![2, 3, 4], vec![1], vec![0; 5]];
	// Move Error 
	// let moved = vec[0];
	// Refrence OK 
	let moved = &vec[0];
	for i in &vec { 
		println!("{:?}", i); 
	}
	println!("ref : {:?}", moved);
}
