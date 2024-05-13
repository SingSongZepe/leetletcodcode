

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		//[[1,1,1],[1,0,1],[1,1,1]]
		let img = vec![
			vec![1,1,1],
			vec![1,0,1],
			vec![1,1,1]
		];
		let result = Solution::image_smoother(img);
		print_matrix(&result);
	}

	#[test]
	fn test2() {
		//[[100,200,100],[200,50,200],[100,200,100]]
		let img = vec![
			vec![100,200,100],
			vec![200,50,200],
			vec![100,200,100]
		];
		let result = Solution::image_smoother(img);
		print_matrix(&result);
	}

	// solution 1
	#[test]
	fn test11() {
		//[[1,1,1],[1,0,1],[1,1,1]]
		let img = vec![
			vec![1,1,1],
			vec![1,0,1],
			vec![1,1,1]
		];
		let result = Solution1::image_smoother(img);
		print_matrix(&result);
	}

	#[test]
	fn test21() {
		//[[100,200,100],[200,50,200],[100,200,100]]
		let img = vec![
			vec![100,200,100],
			vec![200,50,200],
			vec![100,200,100]
		];
		let result = Solution1::image_smoother(img);
		print_matrix(&result);
	}

	// solution 2 one-line
	#[test]
	fn test12() {
		//[[1,1,1],[1,0,1],[1,1,1]]
		let img = vec![
			vec![1,1,1],
			vec![1,0,1],
			vec![1,1,1]
		];
		let result = Solution2::image_smoother(img);
		print_matrix(&result);
	}

	#[test]
	fn test22() {
		//[[100,200,100],[200,50,200],[100,200,100]]
		let img = vec![
			vec![100,200,100],
			vec![200,50,200],
			vec![100,200,100]
		];
		let result = Solution2::image_smoother(img);
		print_matrix(&result);
	}

	#[test]
	fn test_floor() {
		let (a, b) = (1, 4);
		println!("{}", a / b);
	}
}