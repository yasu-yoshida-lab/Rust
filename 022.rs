pub fn get_divisors(n: usize) -> Vec<usize> { 
	let mut res = vec![];

	for i in (1..).take_while(|&x| x * x <= n) { 
		if n % i == 0 { 
			if i * i == n { 
				res.push(i); 
			}
			else { 
				res.push(i);
				res.push(n / i);
			}
		}
	}

	res.sort();
	res
}

fn main() {
	println!("{:?}", get_divisors(30));
}
