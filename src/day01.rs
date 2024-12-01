use std::iter::zip;
use std::fs;

pub fn star1() {
	let content = fs::read_to_string("day01.txt").unwrap();
	let (mut left, mut right): (Vec<i32>, Vec<i32>) = content.split("\n").filter(|&x| !x.is_empty()).map(|line| {
		let parts = line.split(" ").filter(|&x| !x.is_empty()).collect::<Vec<_>>();
		return (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
	}).collect();

	left.sort();
	right.sort();

	let mut total = 0;
	for (l, r) in zip(left, right) {
		total += (l - r).abs();
	}
	println!("{total}");
}

pub fn star2() {
	let content = fs::read_to_string("day01.txt").unwrap();
	let (mut left, mut right): (Vec<i32>, Vec<i32>) = content.split("\n").filter(|&x| !x.is_empty()).map(|line| {
		let parts = line.split(" ").filter(|&x| !x.is_empty()).collect::<Vec<_>>();
		return (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
	}).collect();

	let mut total = 0;
	for val in left {
		total += right.iter().filter(|x| **x == val).count() * (val as usize);
	}
	println!("{total}");
}
