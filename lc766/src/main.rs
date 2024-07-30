mod test;

struct Solution;

impl Solution {
	pub fn is_toeplitz_matrix(mut matrix: Vec<Vec<i32>>) -> bool {
		if matrix.len() == 1 || matrix[0].len() == 1 {
			return true;
		}
		for i in 0..matrix[0].len() - 1 {
			if matrix[0][i] != matrix[1][i+1] {
				return false;
			}
		}
		matrix.remove(0);
		Self::is_toeplitz_matrix(matrix)
	}
}

struct Solution1;

impl Solution1 {
	pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
		fn checker(matrix: &Vec<Vec<i32>>, i: usize) -> bool {
			if i == matrix.len() - 1 {
				return true;
			}
			for j in 0..matrix[0].len() - 1 {
				if matrix[i][j] != matrix[i+1][j+1] {
					return false;
				}
			}
			checker(matrix, i+1)
		}
		if matrix.len() == 1 || matrix[0].len() == 1 {
			return true;
		} else {
			checker(&matrix, 0)
		}
	}
}

fn main() {
    println!("Hello, world!");
}
