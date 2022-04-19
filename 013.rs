use std::collections::HashSet;

fn main() { 
	let mut set = HashSet::new();

	set.insert(0);
	set.insert(1);
	set.insert(2);

	println!("`{}", set.contains(&0));
	println!("`{}", set.contains(&3));

	set.remove(&0);
	println!("`{}", set.contains(&0));
}
