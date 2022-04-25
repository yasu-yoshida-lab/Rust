use std::mem;

#[derive(Clone)]
pub struct UnionFind { 
	parent : Vec<usize>, 
	rank : Vec<usize>, 
	size : Vec<usize>, 
}

impl UnionFind { 
	pub fn new(n: usize) -> Self { 
		UnionFind { 
			parent: (0..n).collect::<Vec<_>>(), 
			rank: vec![0; n], 
			size: vec![1; n], 
		}
	}

	pub fn is_root(&self, x: usize) -> bool { 
		self.parent[x] == x
	}
	pub fn find(&mut self, x: usize) -> usize { 
		if self.is_root(x) { 
			x 
		} 
		else { 
			let root = self.find(self.parent[x]);
			self.parent[x] = root;
			root
		}
	}

	pub fn unin(&mut self, x: usize, y: usize) { 
		let (mut root_x, mut root_y) = (self.find(x), self.find(y));

		if root_x != root_y { 
			if self.rank[root_x] < self.rank[root_y] { 
				mem::swap(&mut root_x, &mut root_y);
			} 
			else if self.rank[root_x] == self.rank[root_y] { 
				self.rank[root_x] += 1; 
			}

			self.parent[root_y] = root_x; 
			self.size[root_x] += self.size[root_y]; 
		}
	}

	pub fn is_same(&mut self, x: usize, y: usize) -> bool { 
		self.find(x) == self.find(y)
	}

	pub fn get_size(&mut self, x: usize) -> usize { 
		if self.is_root(x) { 
			self.size[x]
		}
		else { 
			let root = self.find(x);
			self.size[root]
		}
	}
}

fn main() { 
}
