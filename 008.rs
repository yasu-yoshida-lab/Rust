fn main() {
	let mut v = vec![1, 1, 2, 3, 4, 5, 5];
	v.dedup();
	println!("{:?}", v);

	let mut u = vec![1, 1, 1, 2, 2, 1, 1, 1, 0, 3, 3];
	u.dedup();
	println!("{:?}", u);
}
