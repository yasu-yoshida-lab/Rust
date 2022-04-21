fn main() {
	let vector = vec![30, 20, 30];
	let mut sum = 0;

	for num in &vector { 
		sum += num; 
	}

	println!("{}", sum);
}
