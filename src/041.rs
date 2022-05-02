fn main()
{ 
	let tuple: (String, f64) = ("hello".to_string(), 0.1);
	let hello = tuple.0;
	// Already moved 'tuple.0' Error
	// println!("{}", tuple.0);
	println!("{}", tuple.1);
}
