// fn print<T: std::fmt::Display>(x: T)  or fn print(x: impl std::fmt::Display)
fn print(x: impl std::fmt::Display)
{
	println!("{}", x);
}

fn main() 
{ 
	print(10);
	print("Hello");
}
