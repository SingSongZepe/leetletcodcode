mod test;

trait Average<T> {
	fn make(&self) -> T;
}

impl Average<i32> for (i32, i32) {
	fn make(&self) -> i32 {
		self.0 / self.1
		// 0f32.floor() as i32
	}
}

struct Solution;

impl Solution {
	pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let rs = img.len();
		let cs = img[0].len();

		let mut result = vec![vec![0; cs]; rs];

		for i in 0..rs {
			for j in 0..cs {
				result[i][j] = Self::get_average(&img, i as i32, j as i32);
			}
		}
		result
	}
	pub fn get_average(img: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
		((0.max(i-1) as usize)..=(img.len()-1).min(i as usize+1)).fold((0, 0), |(value, count), x|{
			let (val, cnt) = (0.max(j-1) as usize..=(img[0].len()-1).min(j as usize+1)).fold((0, 0), |(value, count), y|{
				(value + img[x][y], count+1)
			});
			(value + val, count + cnt)
		}).make()
	}
}

struct Solution1;

impl Solution1 {
	pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		(0..img.len()).map(|i| (0..img[0].len()).map(|j| Self::get_average(&img, i as i32, j as i32)).collect()).collect()
	}
	pub fn get_average(img: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
		((0.max(i-1) as usize)..=(img.len()-1).min(i as usize+1)).fold((0, 0), |(value, count), x|{
			let (val, cnt) = (0.max(j-1) as usize..=(img[0].len()-1).min(j as usize+1)).fold((0, 0), |(value, count), y|{
				(value + img[x][y], count+1)
			});
			(value + val, count + cnt)
		}).make()
	}
}

struct Solution2;

impl Solution2 {
	pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		(0..img.len()).map(|i| (0..img[0].len()).map(|j|
			((0.max(i as i32-1) as usize)..=(img.len()-1).min(i+1)).fold((0, 0), |(value, count), x|{
				let (val, cnt) = (0.max(j as i32 -1) as usize..=(img[0].len()-1).min(j+1)).fold((0, 0), |(value, count), y|{
					(value + img[x][y], count+1)
				});
				(value + val, count + cnt)
			}).make()
		).collect()).collect()
	}
}


fn print_matrix(matrix: &Vec<Vec<i32>>) {
	for row in matrix {
		println!("{:?}", row);
	}
	println!("");
}

fn main() {
    println!("Hello, world!");
}
