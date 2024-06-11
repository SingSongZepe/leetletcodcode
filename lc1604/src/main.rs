mod test;

struct Solution;
use std::collections::{
	HashMap,
	HashSet,
};

impl Solution {
	pub fn alert_names(key_name: Vec<String>, mut key_time: Vec<String>) -> Vec<String> {
		let mut tn: Vec<(String, i32)> = key_time.into_iter().enumerate().map(|(idx, t)| {
			(key_name[idx].to_owned(), t.replace(":", "").parse().unwrap())
		}).collect::<Vec<_>>();
		tn.sort_by_key(|t| t.1);

		let mut hm = HashMap::<String, Vec<i32>>::new();
		let mut res = HashSet::new();

		for (name, time) in tn {
			if res.contains(&name) {
				continue;
			}
			hm.entry(name.clone()).or_insert(vec![]).push(time);
			if let Some(times) = hm.get_mut(&name) {
				if times.len() > 2 {
					if times[times.len()-1] - times[times.len()-3] <= 100 {
						res.insert(name.clone());
					}
				}
			}
		}

		let mut res: Vec<String> = res.into_iter().collect();
		res.sort();
		return res;
	}
}


struct Solution1;

impl Solution1 {
	fn alert_names(names: Vec<String>, times: Vec<String>) -> Vec<String> {
		let mut times: Vec<(usize, i32)> = times.iter().enumerate().map(|(idx, t)| (idx, t.replace(":", "").parse().unwrap())).collect();
		times.sort_by(|a, b| a.1.cmp(&b.1));

		let mut mp: HashMap<String, Vec<i32>> = HashMap::new();
		let mut res: HashSet<String> = HashSet::new();

		for (idx, t) in times {
			if res.contains(&names[idx]) {
				continue;
			}
			let name = &names[idx];
			mp.entry(name.to_string()).or_insert(Vec::new()).push(t);
			if mp[name].len() > 2 {
				if mp[name][mp[name].len()-1] - mp[name][mp[name].len()-3] <= 100 {
					res.insert(name.to_string());
				}
			}
		}

		let mut sorted_res: Vec<String> = res.into_iter().collect();
		sorted_res.sort();

		sorted_res
	}
}

struct Solution2;

impl Solution2 {
	pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
		let mut map = std::collections::HashMap::new();

		for (name, time) in key_name.iter().zip(key_time.iter()) {
			let time = time
				.split(':')
				.map(|s| s.parse::<i32>().unwrap())
				.collect::<Vec<_>>();

			let time = time[0] * 60 + time[1];

			map.entry(name).or_insert(vec![]).push(time);
		}

		let mut res = vec![];

		for (name, times) in map {
			let mut times = times;

			times.sort();

			for (i, j) in (2..times.len()).enumerate() {
				if times[j] - times[i] <= 60 {
					res.push(name.to_string());
					break;
				}
			}
		}

		res.sort();

		res
	}
}


fn helper(vstr: Vec<&str>) -> Vec<String> {
    let mut v = Vec::new();
    for s in vstr {
		v.push(s.to_string());
	}
	return v;
}


fn main() {
    println!("Hello, world!");
}
