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

	let total = zip(left, right).map(|(l, r)| (l - r).abs()).sum::<i32>();
	println!("{total}");
}

pub fn star2() {
	let content = fs::read_to_string("day01.txt").unwrap();
	let (mut left, mut right): (Vec<i32>, Vec<i32>) = content.split("\n").filter(|&x| !x.is_empty()).map(|line| {
		let parts = line.split(" ").filter(|&x| !x.is_empty()).collect::<Vec<_>>();
		return (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
	}).collect();

	let total = left.iter().map(|val| right.iter().filter(|x| **x == *val).count() * (*val as usize)).sum::<usize>();
	println!("{total}");
}
