use regex::Regex;
use std::fs;

pub fn star1() {
	let content = fs::read_to_string("day03.txt").unwrap();
	let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

	let total: u32 = re.captures_iter(&content).map(|c| {
		let (_, [one, two]) = c.extract();
		let one = one.parse::<u32>().unwrap();
		let two = two.parse::<u32>().unwrap();
		one * two
	}).sum();
	println!("{total}");
}

pub fn star2() {
	let content = fs::read_to_string("day03.txt").unwrap();
	let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

	let total = content.split("do()").map(|substr| {
		let mut splitter = substr.split("don't()");
		if let Some(text) = splitter.next() {
			re.captures_iter(&text).map(|c| {
				let (_, [one, two]) = c.extract();
				let one = one.parse::<u32>().unwrap();
				let two = two.parse::<u32>().unwrap();
				one * two
			}).sum::<u32>()
		} else {
			0
		}
	}).sum::<u32>();
	println!("{total}");
}
