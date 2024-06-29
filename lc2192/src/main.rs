mod test;

struct Solution;

#[derive(Clone)]
struct Bits {
	vec: Vec<u64>
}

impl Bits {
	fn new(n: usize) -> Self {
		Self{vec: vec![0; (n+63)/64]}
	}
	fn set(&mut self, ix: usize) {
		self.vec[ix/64] |= 1<<(ix%64);
	}
	fn merge(&mut self, other: &Bits) {
		for i in 0..other.vec.len() {
			self.vec[i] |= other.vec[i];
		}
	}
	fn as_vec(&self) -> Vec<i32> {
		let mut i = 0_i32;
		let mut ans = Vec::new();
		for j in 0..self.vec.len() {
			let mut val = self.vec[j];
			while val != 0 {
				let lsb = ((val as i64) & -(val as i64)) as u64;
				ans.push(i + lsb.trailing_zeros() as i32);
				val ^= lsb;
			}
			i += 64;
		}
		ans
	}
}

impl Solution {
	fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<Option<Bits>>, u: usize) {
		let mut bits = Bits::new(graph.len());
		for &v in &graph[u] {
			if seen[v].is_none() {
				Self::dfs(graph, seen, v);
			}
			bits.set(v);
			bits.merge(seen[v].as_ref().unwrap());
		}
		seen[u] = Some(bits);
	}

	pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let n = n as usize;
		let mut graph = vec![Vec::new(); n];
		for edge in edges {
			let u = edge[0] as usize;
			let v = edge[1] as usize;
			graph[v].push(u);
		}

		const no_bits : Option<Bits> = None;
		let mut seen = vec![no_bits; n];
		let mut ans = Vec::with_capacity(n);
		for i in 0..n {
			if seen[i].is_none() {
				Self::dfs(&graph, &mut seen, i);
			}
			ans.push(seen[i].as_ref().unwrap().as_vec());
		}
		ans
	}
}

fn main() {
    println!("Hello, world!");
}
