use std::fs;

pub fn star1() {
	let content = fs::read_to_string("day04.txt").unwrap();
	let lines = content.split("\n").collect::<Vec<_>>();
	let max = lines.len() - 1;
	let line_len = lines[0].len();
	let new_line_len = line_len + max;

	let mut total = 0;

	let mut lines_diagonal_left_up = vec![" ".repeat(line_len).to_string(); new_line_len];
	let mut lines_diagonal_bottom_down = vec![" ".repeat(line_len).to_string(); new_line_len];
	let mut lines_vertical = vec![" ".repeat(line_len).to_string(); line_len];

	for (row, line) in lines.iter().enumerate() {
		total += line.matches("XMAS").count();
		total += line.matches("SAMX").count();

		for (col, c) in line.bytes().enumerate() {
			let s = &format!("{}", c as char);
			
			lines_diagonal_left_up[line_len - 1 - col + row].replace_range(col..(col+1), s);
			lines_diagonal_bottom_down[col + row].replace_range(col..(col+1), s);
			lines_vertical[col].replace_range(row..(row+1), s);
		}
	}

	for line in lines_diagonal_left_up {
		total += line.matches("XMAS").count();
		total += line.matches("SAMX").count();
	}

	for line in lines_diagonal_bottom_down {
		total += line.matches("XMAS").count();
		total += line.matches("SAMX").count();
	}

	for line in lines_vertical {
		total += line.matches("XMAS").count();
		total += line.matches("SAMX").count();
	}

	println!("{total}");
}

pub fn star2() {
	let content = fs::read_to_string("day04.txt").unwrap();
	let mut lines = content.split("\n").map(|s| {
		let mut v = vec![0u8; 1];
		v.extend(s.as_bytes().to_vec());
		v.extend_from_slice(&[0u8; 1]);
		v
	}).collect::<Vec<Vec<u8>>>();
	lines.push(vec![0u8; lines[0].len()]);
	lines.insert(0, vec![0u8; lines[0].len()]);
	let height = lines.len() - 1;
	let width = lines[0].len() - 1;

	let mut total = 0;
	for y in 1..height {
		for x in 1..width {
			if lines[y][x] != b'A' {
				continue;
			}

			if lines[y-1][x-1] == lines[y+1][x+1] {
				continue;
			}

			// M.M
			// .A.
			// S.S
			if lines[y-1][x-1] == lines[y-1][x+1]
				&& lines[y+1][x-1] == lines[y+1][x+1] {

				if lines[y-1][x-1] == b'M' && lines[y+1][x-1] == b'S' {
					total += 1;
				}
				if lines[y-1][x-1] == b'S' && lines[y+1][x-1] == b'M' {
					total += 1;
				}
			}
			// M.S
			// .A.
			// M.S
			if lines[y-1][x-1] == lines[y+1][x-1]
				&& lines[y-1][x+1] == lines[y+1][x+1] {

				if lines[y-1][x-1] == b'M' && lines[y-1][x+1] == b'S' {
					total += 1;
				}
				if lines[y-1][x-1] == b'S' && lines[y-1][x+1] == b'M' {
					total += 1;
				}
			}
		}
	}
	
	println!("{total}");
}
