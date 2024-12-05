use std::collections::HashMap;
use std::fs;

pub fn star1_2() {
	let content = fs::read_to_string("day05.txt").unwrap();
	let mut iter = content.split("\n");
	let mut rules = HashMap::new();
	for rule in iter.clone().take_while(|x| !x.is_empty()) {
		let data = rule.split('|').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
		if !rules.contains_key(&data[0]) {
			rules.insert(data[0], vec![]);
		}

		rules.get_mut(&data[0]).unwrap().push(data[1]);
	}

	let mut total = 0;
	let mut total_err = 0;
	for seq in iter.clone().skip_while(|x| !x.is_empty()).filter(|x| !x.is_empty()) {
		let vec = seq.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
		let rev = vec.iter().rev().map(|x| *x).collect::<Vec<u32>>();

		let total_sum = get_ordered_sum(&rev, &rules);

		if total_sum == 0 {
			total += vec[vec.len() / 2];
		} else {
			total_err += fix_unordered(&vec, &rules)
		}
	}

	println!("{total}");
	println!("{total_err}");
}

fn fix_unordered(vec: &Vec<u32>, rules: &HashMap::<u32, Vec<u32>>) -> u32 {
	let mut ordered = vec![vec[0]];
	for value in vec.iter().skip(1) {
		let mut inserted = false;
		for i in 0..ordered.len() {
			if let Some(follow) = rules.get(&value) {
				if follow.contains(&ordered[i]) {
					ordered.insert(i, *value);
					inserted = true;
					break;
				}
			}
		}

		if !inserted {
			ordered.push(*value);
		}
	}

	ordered[ordered.len() / 2]
}

fn get_ordered_sum(rev: &Vec<u32>, rules: &HashMap::<u32, Vec<u32>>) -> u32 {
	let mut total_sum = 0;
	for i in 1..rev.len() {
		let current = &rev[i-1];
		let subvec = &rev[i..];
		if let Some(prec) = rules.get(&current) {
			let sum = prec.iter().map(|x| if subvec.contains(&x) { 1 } else { 0 }).sum::<u32>();
			total_sum += sum;
		}
	}
	total_sum
}