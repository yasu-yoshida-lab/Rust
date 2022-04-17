use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
pub struct Edge<T> { 
	pub src: usize, 
	pub dst: usize, 
	pub weight: T, 
}

impl <T> Ord for Edge <T> where T : Ord, { 
	fn cmp(&self, other : &Self) -> Ordering { 
		self.weight.cmp(&other.weight)
	}
}

impl<T> PartialOrd for Edge<T> where T : Ord, { 
	fn partial_cmp(&self, other : &Self) -> Option<Ordering> { 
		Some(self.cmp(other))
	}
}

#[derive(Clone)]
pub struct Graph<T> { 
	pub edges : Vec<Vec<Edge<T>>>, 
}

impl<T> Graph<T> where T : Clone, { 
	pub fn new(size : usize) -> Self { 
		Graph {
			edges : vec![vec![]; size], 
		}
	}

	pub fn size(&self) -> usize { 
		self.edges.len()
	}

	pub fn add_edge(&mut self, src : usize, dst : usize, weight : T) { 
		self.edges[src].push(Edge { src, dst, weight } );
	}
}

fn main() { 
}
