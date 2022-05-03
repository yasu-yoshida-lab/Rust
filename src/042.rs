fn sum(v: Vec<i32>) -> i32
{ 
	let mut ret = 0;
	for &i in &v { 
		ret += i;
	}
	return ret;
}

fn main() 
{ 
	let vec = vec![20, 80, 50, 40];
	let s = sum(vec);
	// s = sum(vec)の時点でvecは所有権を失っている
	println!("{}", s);
	// vecを出力しようとするとエラーとなる
	// println!("{:?}", vec);
}
