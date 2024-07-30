

#[cfg(test)]
mod tests {
	use crate::*;
	
	#[test]
	fn test1() {
		let obj = FoodRatings::new(foods, cuisines, ratings);
		obj.change_rating(food, newRating);
		let ret_2: String = obj.highest_rated(cuisine);
	}

}