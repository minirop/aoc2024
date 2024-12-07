use std::fs;

pub fn star1_2() {
	let content = fs::read_to_string("day07.txt").unwrap();
	let operations = content.split("\n").map(|line| {
		let l = line.split(':').collect::<Vec<_>>();
		let res = l[0].parse::<u64>().unwrap();
		let values = l[1].split(' ').filter(|x| !x.is_empty()).map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
		(res, values)
	}).collect::<Vec<(_, Vec<u64>)>>();

	let total = operations.iter().map(|(res, values)| if calculate(*res, values.clone()) {
		*res
	} else {
		0
	}).sum::<u64>();

	println!("{total}");
}

fn calculate(result: u64, numbers: Vec<u64>) -> bool {
	calculate_impl(result, 0, numbers)
}

fn calculate_impl(result: u64, current: u64, numbers: Vec<u64>) -> bool {
	if numbers.len() > 0 {
		let first = numbers[0];
		let mut numbers = numbers;
		numbers.remove(0);

		if calculate_impl(result, current + first, numbers.clone()) {
			return true;
		}

		if calculate_impl(result, current * first, numbers.clone()) {
			return true;
		}
		
		// day 02 only
		if calculate_impl(result, format!("{}{}", current, first).parse::<u64>().unwrap(), numbers.clone()) {
			return true;
		}
		
		false
	} else {
		result == current
	}
}
