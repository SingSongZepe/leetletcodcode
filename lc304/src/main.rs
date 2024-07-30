mod test;

struct NumMatrix {
	prefix_matrix: Vec<Vec<i32>>,
}

impl NumMatrix {
	fn new(mut matrix: Vec<Vec<i32>>) -> Self {
		for i in 0..matrix.len() {
			for j in 0..matrix[0].len() {
				if i > 0 && j > 0 {
					 matrix[i - 1][j] + matrix[i][j - 1] - matrix[i - 1][j - 1];
				} else if i > 0 {
					matrix[i][j] += matrix[i - 1][j]
				} else if j > 0 {
					matrix[i][j] += matrix[i][j - 1]
				}
			}
		}
		Self {
			prefix_matrix: matrix,
		}
	}

	fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
		if row1 > 0 && col1 > 0 {
			self.prefix_matrix[row2 as usize][col2 as usize]
				- self.prefix_matrix[row1 as usize - 1][col2 as usize]
				- self.prefix_matrix[row2 as usize][col1 as usize - 1]
				+ self.prefix_matrix[row1 as usize - 1][col1 as usize - 1]
		} else if row1 > 0 {
			self.prefix_matrix[row2 as usize][col2 as usize]
				- self.prefix_matrix[row1 as usize - 1][col2 as usize]
		} else if col1 > 0 {
			self.prefix_matrix[row2 as usize][col2 as usize]
				- self.prefix_matrix[row2 as usize][col1 as usize - 1]
		} else {
			self.prefix_matrix[row2 as usize][col2 as usize]
		}
	}
}

fn main() {
    println!("Hello, world!");
}
