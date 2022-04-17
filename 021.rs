pub fn is_prime(n : usize) -> bool { 
	if n == 0 || n == 1 { 
		return false;
	}

	for i in (2..).take_while(|&x| x * x <= n) { 
		if n % i == 0 { 
			return false; 
		}
	}
	
	return true; 
}

fn main() { 
	println!("{}", is_prime(10));
	println!("{}", is_prime(11));
}
