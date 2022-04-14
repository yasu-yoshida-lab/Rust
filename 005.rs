fn main() { 
	let mut v = vec![3.0, 1.0, 4.0, 1.0, 5.0];
	v.sort_by(|a, b| a.partial_cmp(b).unwrap());
	println!("{:?}", v);
}
