use std::collections::HashMap;

fn main() { 
	let mut map = HashMap::new();

	map.insert("zero", 0);
	map.insert("one", 1);
	map.insert("two", 2);

	println!("{:?}", map.get("zero"));
	println!("{:?}", map.get("three"));
}
