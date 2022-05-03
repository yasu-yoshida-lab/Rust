fn sum(v: &Vec<i32>) -> i32 
{
	let mut ret = 0;
	for &i in v { 
		ret += i;
	}
	return ret;
}

fn main() { 
	let vec = vec![20, 80, 50, 40];
	let s = sum(&vec);
	println!("{:?} is sum : {}", vec, s);
}
