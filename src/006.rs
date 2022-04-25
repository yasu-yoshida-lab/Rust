fn main() {
	let mut v = vec![(3, 1, 4), (1, 5, 9), (2, 6, 5)];

	v.sort_by_key(|x| x.0);
	println!("{:?}", v);

	v.sort_by_key(|x| x.1);
	println!("{:?}", v);

	v.sort_by_key(|x| x.2);
	println!("{:?}", v);
}
