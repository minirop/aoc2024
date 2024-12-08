use std::fs;

pub fn star1() {
	let content = fs::read_to_string("day08.txt").unwrap();
	let map = content.split("\n").collect::<Vec<_>>();
	let width = map[0].len();
	let height = map.len();
	let mut map = map.join("").as_bytes().to_vec();

	let mut phantom = 0;
	for i in 0..map.len() {
		let curr = map[i];
		if curr != b'.' && curr != b'#' {
			for j in (i + 1)..map.len() {
				if curr == map[j] {
					let x1 = (i % width) as i64;
					let y1 = (i / width) as i64;
					let x2 = (j % width) as i64;
					let y2 = (j / width) as i64;

					let off_x = (x2 - x1);
					let off_y = (y2 - y1);

					phantom += update_map(&mut map, width, height, x2 + off_x, y2 + off_y);
					phantom += update_map(&mut map, width, height, x1 - off_x, y1 - off_y);
				}
			}
		}
	}

	let total = map.iter().filter(|&n| *n == b'#').count();
	println!("{}", total + phantom);
}

fn update_map(map: &mut Vec<u8>, width: usize, height: usize, x: i64, y: i64) -> usize {
	if x < 0 || x >= (width as i64) || y < 0 || y >= (height as i64) {
		return 0;
	}

	let index = (x as usize) + (y as usize) * width;

	if map[index] == b'.' {
		map[index] = b'#';
	} else if map[index] != b'#' {
		return 1;
	}

	0
}

pub fn star2() {
	let content = fs::read_to_string("day08.txt").unwrap();
	let map = content.split("\n").collect::<Vec<_>>();
	let width = map[0].len();
	let height = map.len();
	let mut map = map.join("").as_bytes().to_vec();

	for i in 0..map.len() {
		let curr = map[i];
		if curr != b'.' && curr != b'#' {
			for j in (i + 1)..map.len() {
				if curr == map[j] {
					let x1 = (i % width) as i64;
					let y1 = (i / width) as i64;
					let x2 = (j % width) as i64;
					let y2 = (j / width) as i64;

					let off_x = (x2 - x1);
					let off_y = (y2 - y1);

					update_map_all(&mut map, width, height, x2, off_x, y2, off_y);
					update_map_all(&mut map, width, height, x1, -off_x, y1, -off_y);
				}
			}
		}
	}

	let total = map.len() - map.iter().filter(|&n| *n == b'.').count();
	println!("{}", total);
}

fn update_map_all(map: &mut Vec<u8>, width: usize, height: usize, x: i64, off_x: i64, y: i64, off_y: i64) {
	let mut x = x;
	let mut y = y;

	loop {
		if x < 0 || x >= (width as i64) || y < 0 || y >= (height as i64) {
			break;
		}

		let index = (x as usize) + (y as usize) * width;

		if map[index] == b'.' {
			map[index] = b'#';
		}

		x += off_x;
		y += off_y;
	}
}
