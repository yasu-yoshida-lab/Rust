use std::collections::BinaryHeap;

fn main() { 
	let mut que = BinaryHeap::new();

	que.push(0);
	que.push(2);
	que.push(1);

	println!("{:?}", que.pop());
	println!("{:?}", que.pop());
	que.push(3);
	println!("{:?}", que.pop());
	println!("{:?}", que.pop());
	println!("{:?}", que.pop());
}
