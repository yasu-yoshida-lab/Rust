fn main() { 
	let mut v = vec![0, 1, 2, 3, 4];
	println!("{}", v.len());
	println!("{:?}", v);

	v[2] = 6;
	println!("{:?}", v);
}
