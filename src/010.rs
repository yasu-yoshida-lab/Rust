fn main() { 
	let mut stack = vec![];

	stack.push(0);
	stack.push(1);
	stack.push(2);

	println!("{:?}", stack);

	println!("{:?}", stack.pop().unwrap());
	println!("{:?}", stack);

}
