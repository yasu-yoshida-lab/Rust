fn main() 
{
	let vec = vec![1, 2, 3];
	let reference = &vec;
	// moved error
	// let moved = vec;
	println!("{:?}", reference);
}
