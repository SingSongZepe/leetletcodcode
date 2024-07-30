mod test;

struct Solution;

impl Solution {
	pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
		let mut max_area: f64 = 0.0;
		for i in 0..points.len() {
			for j in i+1..points.len() {
				for k in j+1..points.len() {
					max_area = max_area.max(
						0.5f64 * (
							(points[i][0]-points[j][0])*(points[i][1]+points[j][1])
							+ (points[j][0]-points[k][0])*(points[j][1]+points[k][1])
							+ (points[k][0]-points[i][0])*(points[k][1]+points[i][1])
						).abs() as f64
					);
				}
			}
		}
		max_area
	}
}

fn main() {
    println!("Hello, world!");
}
