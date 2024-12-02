use std::fs;

pub fn star1() {
	let content = fs::read_to_string("day02.txt").unwrap();
	let total: i32 = content.split("\n").map(|line| {
		let report = line.split(" ").map(|elem| elem.parse::<i32>().unwrap()).collect::<Vec<_>>();
		is_report_safe(&report)
	}).sum();
	println!("{total}");
}

pub fn star2() {
	let content = fs::read_to_string("day02.txt").unwrap();
	let total: i32 = content.split("\n").map(|line| {
		let report = line.split(" ").map(|elem| elem.parse::<i32>().unwrap()).collect::<Vec<_>>();
		if is_report_safe(&report) == 1 {
			1
		} else {
			for i in 0..report.len() {
				let mut new_report = report.clone();
				new_report.remove(i);
				if is_report_safe(&new_report) == 1 {
					return 1;
				}
			}

			0
		}
	}).sum();
	println!("{total}");
}

fn is_report_safe(report: &Vec<i32>) -> i32 {
	let mut increasing = false;
	let mut decreasing = false;
	for elems in report.windows(2) {
		let a = &elems[0];
		let b = &elems[1];
		if a == b || a.overflowing_sub(*b).0.abs() > 3 {
			return 0;
		} else if !increasing && b > a {
			increasing = true;
		} else if !decreasing && b < a {
			decreasing = true;
		}
	}

	if increasing && decreasing { 0 } else { 1 }
}
