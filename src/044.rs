fn double(x: &mut i32)
{ 
	*x *= 2;
}

fn main() 
{ 
	let mut hoge = 10;
	double(&mut hoge); 
	println!("hoge : {}", hoge);
	double(&mut hoge); 
	println!("hoge : {}", hoge);
}
