fn func(n: i32) -> i32 
{
	if n == 0 { 
		return 1;
	} 
	else { 
		return func(n - 1) * n;
	}
}

fn main()
{ 
	let val: i32 = func(3);
	println!("ret : {}", val);
}
