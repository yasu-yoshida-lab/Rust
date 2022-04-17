use std::ops;

const MOD : usize = 1000000007;

#[derive(Copy, Clone)]
pub struct ModInt { 
	value : usize, 
}

impl ModInt { 
	pub fn new(value : usize) -> ModInt { 
		ModInt { value : value % MOD } 
	}

	pub fn value(&self) -> usize { 
		self.value 
	} 

	pub fn inverse(&self) -> ModInt { 
		fn extend_gcd(a : isize, b : isize) -> (isize, isize) { 
			if (a, b) == (1, 0) { 
				return (1, 0); 
			}
			let (x, y) = extend_gcd(b, a % b);
			(y, x - (a / b) * y) 
		}

		let (x, _y) = extend_gcd(self.value() as isize, MOD as isize); 
		ModInt::new((MOD as isize + x) as usize)
	}
}

impl ops::Add for ModInt { 
	type Output = ModInt; 
	fn add(self, other : Self) -> Self { 
		ModInt::new(self.value + other.value)
	}
}

impl ops::Sub for ModInt { 
	type Output = ModInt; 
	fn sub(self, other : Self) -> Self { 
		ModInt::new(MOD + self.value - other.value)
	}
}

impl ops::Mul for ModInt { 
	type Output = ModInt; 
	fn mul(self, other : Self) -> Self { 
		ModInt::new(self.value * other.value)
	}
}

impl ops::AddAssign for ModInt { 
	fn add_assign(&mut self, other : Self) { 
		*self = *self + other;
	}
}

impl ops::SubAssign for ModInt { 
	fn sub_assign(&mut self, other : Self) { 
		*self = *self - other;
	}
}

impl ops::MulAssign for ModInt { 
	fn mul_assign(&mut self, other : Self) { 
		*self = *self * other; 
	}
}


impl ops::Div for ModInt { 
	type Output = ModInt;
	fn div(self, other : Self) -> Self { 
		self * other.inverse()
	}
}

impl ops::DivAssign for ModInt { 
	fn div_assign(&mut self, other : Self) { 
		*self = *self / other;
	}
}

pub fn pow(a : usize, mut n : usize) -> ModInt { 
	let mut res = ModInt::new(1);
	let mut x = ModInt::new(a);
	while n > 0 { 
		if n % 2 == 1 { 
			res *= x;
		} 
		x = x * x;
		n /= 2;
	}
	res 
}

pub fn factorial(n : usize) -> ModInt { 
	(1..=n).fold(ModInt::new(1), |x, y| x * ModInt::new(y))
}

pub fn permutation(n : usize, r : usize) -> ModInt { 
	(0..r).fold(ModInt::new(1), |x, y| x * ModInt::new(n - y))
}

pub fn combination(n : usize, r : usize) -> ModInt { 
	permutation(n, r) / factorial(r)
}

fn main() { 
	let mut a = ModInt::new(1000000000);
	a += ModInt::new(9);
	println!("{}", a.value());

	println!("{}", ModInt::new(2).inverse().value());
	println!("{}", ModInt::new(3).inverse().value());
}
