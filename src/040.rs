fn main() 
{
	let hello = "hello".to_string();
	let tuple: (String, f64) = (hello, 0.1);
	// Already moved 'hello'
	// moved error
	// println!("{}", hello);
}
