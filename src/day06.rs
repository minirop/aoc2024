use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const NORTH: u32 = 0;
const EAST: u32 = 1;
const SOUTH: u32 = 2;
const WEST: u32 = 3;
const DIR_COUNT: u32 = 4;

pub fn star1() {
	let content = fs::read_to_string("day06.txt").unwrap();
	let maze = content.split("\n").collect::<Vec<_>>();
	let width = maze[0].len();
	let height = maze.len();
	let mut maze = maze.join("").as_bytes().to_vec();

	let mut caret = maze.iter().position(|&r| r == b'^').unwrap();
	maze[caret] = b'x';
	let mut direction = NORTH;

	loop {
		let next_pos = match direction {
			NORTH => {
				let (next_pos, ov) = caret.overflowing_sub(width);
				if ov { break; }

				if maze[next_pos] == b'#' {
					direction = EAST;
					caret
				} else {
					next_pos
				}
			},
			EAST => {
				let (next_pos, ov) = caret.overflowing_add(1);
				if ov { break; }

				let guard_y_prev = caret / width;
				let guard_y_next = next_pos / width;
				if guard_y_prev != guard_y_next {
					break;
				}

				if maze[next_pos] == b'#' {
					direction = SOUTH;
					caret
				} else {
					next_pos
				}
			},
			SOUTH => {
				let (next_pos, ov) = caret.overflowing_add(width);
				if ov || next_pos >= maze.len() { break; }

				if maze[next_pos] == b'#' {
					direction = WEST;
					caret
				} else {
					next_pos
				}
			},
			WEST => {
				let (next_pos, ov) = caret.overflowing_sub(1);
				if ov { break; }

				let guard_y_prev = caret / width;
				let guard_y_next = next_pos / width;
				if guard_y_prev != guard_y_next {
					break;
				}

				if maze[next_pos] == b'#' {
					direction = NORTH;
					caret
				} else {
					next_pos
				}
			},
			_ => todo!(), 
		};

		maze[next_pos] = b'x';
		caret = next_pos;
	}

	let steps = maze.iter().filter(|&n| *n == b'x').count();
	println!("{steps}", );
}

pub fn star2_old() {
	let content = fs::read_to_string("day06.txt").unwrap();
	let maze = content.split("\n").collect::<Vec<_>>();
	let width = maze[0].len();
	let height = maze.len();
	let mut maze = maze.join("").as_bytes().to_vec();

	let mut caret = maze.iter().position(|&r| r == b'^').unwrap();
	let starting_point = caret;
	maze[caret] = b'.';
	let mut direction = NORTH;

	let mut tests = HashSet::new();
	tests.insert(caret);

	loop {
		let next_pos = match direction {
			NORTH => {
				let (next_pos, ov) = caret.overflowing_sub(width);
				if ov { break; }

				if maze[next_pos] == b'#' {
					direction = EAST;
					caret
				} else {
					next_pos
				}
			},
			EAST => {
				let (next_pos, ov) = caret.overflowing_add(1);
				if ov { break; }

				let guard_y_prev = caret / width;
				let guard_y_next = next_pos / width;
				if guard_y_prev != guard_y_next {
					break;
				}

				if maze[next_pos] == b'#' {
					direction = SOUTH;
					caret
				} else {
					next_pos
				}
			},
			SOUTH => {
				let (next_pos, ov) = caret.overflowing_add(width);
				if ov || next_pos >= maze.len() { break; }

				if maze[next_pos] == b'#' {
					direction = WEST;
					caret
				} else {
					next_pos
				}
			},
			WEST => {
				let (next_pos, ov) = caret.overflowing_sub(1);
				if ov { break; }

				let guard_y_prev = caret / width;
				let guard_y_next = next_pos / width;
				if guard_y_prev != guard_y_next {
					break;
				}

				if maze[next_pos] == b'#' {
					direction = NORTH;
					caret
				} else {
					next_pos
				}
			},
			_ => todo!(), 
		};

		tests.insert(next_pos);
		caret = next_pos;
	}

	let mut total = 0;
	// println!("steps: {}", tests.len());
	for test in tests {
		// println!("doing test: {test}", );
		let mut visited = HashMap::new();

		caret = starting_point;
		direction = NORTH;
		maze[test] = b'#';
		loop {
			let next_pos = match direction {
				NORTH => {
					let (next_pos, ov) = caret.overflowing_sub(width);
					if ov { break; }

					if maze[next_pos] == b'#' {
						direction = EAST;
						caret
					} else {
						next_pos
					}
				},
				EAST => {
					let (next_pos, ov) = caret.overflowing_add(1);
					if ov { break; }

					let guard_y_prev = caret / width;
					let guard_y_next = next_pos / width;
					if guard_y_prev != guard_y_next {
						break;
					}

					if maze[next_pos] == b'#' {
						direction = SOUTH;
						caret
					} else {
						next_pos
					}
				},
				SOUTH => {
					let (next_pos, ov) = caret.overflowing_add(width);
					if ov || next_pos >= maze.len() { break; }

					if maze[next_pos] == b'#' {
						direction = WEST;
						caret
					} else {
						next_pos
					}
				},
				WEST => {
					let (next_pos, ov) = caret.overflowing_sub(1);
					if ov { break; }

					let guard_y_prev = caret / width;
					let guard_y_next = next_pos / width;
					if guard_y_prev != guard_y_next {
						break;
					}

					if maze[next_pos] == b'#' {
						direction = NORTH;
						caret
					} else {
						next_pos
					}
				},
				_ => todo!(), 
			};

			if let Some(already) = visited.get(&next_pos) {
				if *already == direction {
					// panic!("FOUND!!!");
					total += 1;
					break;
				}
			}
			visited.insert(next_pos, direction);
			caret = next_pos;
		}

		// for y in 0..width {
		// 	for x in 0..height {
		// 		print!("{}", maze[x + y * width] as char);
		// 	}
		// 	println!("");
		// }
		// println!("{:?}", visited);
		// println!("");

		maze[test] = b'.';
	}

	println!("{total}");
}

pub fn star2() {
	let content = fs::read_to_string("day06.txt").unwrap();
	let maze = content.split("\n").collect::<Vec<_>>();
	let width = maze[0].len();
	let height = maze.len();
	let mut maze = maze.join("").as_bytes().to_vec();

	let mut caret = maze.iter().position(|&r| r == b'^').unwrap();
	let starting_point = caret;
	maze[caret] = b'.';
	let mut direction = NORTH;

	let mut tests = HashSet::new();
	tests.insert(caret);

	loop {
		let next_pos = match direction {
			NORTH => {
				let (next_pos, ov) = caret.overflowing_sub(width);
				if ov { break; }

				if maze[next_pos] == b'#' {
					direction = EAST;
					caret
				} else {
					next_pos
				}
			},
			EAST => {
				let (next_pos, ov) = caret.overflowing_add(1);
				if ov { break; }

				let guard_y_prev = caret / width;
				let guard_y_next = next_pos / width;
				if guard_y_prev != guard_y_next {
					break;
				}

				if maze[next_pos] == b'#' {
					direction = SOUTH;
					caret
				} else {
					next_pos
				}
			},
			SOUTH => {
				let (next_pos, ov) = caret.overflowing_add(width);
				if ov || next_pos >= maze.len() { break; }

				if maze[next_pos] == b'#' {
					direction = WEST;
					caret
				} else {
					next_pos
				}
			},
			WEST => {
				let (next_pos, ov) = caret.overflowing_sub(1);
				if ov { break; }

				let guard_y_prev = caret / width;
				let guard_y_next = next_pos / width;
				if guard_y_prev != guard_y_next {
					break;
				}

				if maze[next_pos] == b'#' {
					direction = NORTH;
					caret
				} else {
					next_pos
				}
			},
			_ => todo!(), 
		};

		tests.insert(next_pos);
		caret = next_pos;
	}

	let mut total = 0;
	// println!("steps: {}", tests.len());
	for test in tests {
		// println!("doing test: {test}", );
		let mut maze = maze.clone();
		thread::spawn(move || {
			let mut visited = HashMap::new();

			caret = starting_point;
			direction = NORTH;
			maze[test] = b'#';
			loop {
				let next_pos = match direction {
					NORTH => {
						let (next_pos, ov) = caret.overflowing_sub(width);
						if ov { break; }

						if maze[next_pos] == b'#' {
							direction = EAST;
							caret
						} else {
							next_pos
						}
					},
					EAST => {
						let (next_pos, ov) = caret.overflowing_add(1);
						if ov { break; }

						let guard_y_prev = caret / width;
						let guard_y_next = next_pos / width;
						if guard_y_prev != guard_y_next {
							break;
						}

						if maze[next_pos] == b'#' {
							direction = SOUTH;
							caret
						} else {
							next_pos
						}
					},
					SOUTH => {
						let (next_pos, ov) = caret.overflowing_add(width);
						if ov || next_pos >= maze.len() { break; }

						if maze[next_pos] == b'#' {
							direction = WEST;
							caret
						} else {
							next_pos
						}
					},
					WEST => {
						let (next_pos, ov) = caret.overflowing_sub(1);
						if ov { break; }

						let guard_y_prev = caret / width;
						let guard_y_next = next_pos / width;
						if guard_y_prev != guard_y_next {
							break;
						}

						if maze[next_pos] == b'#' {
							direction = NORTH;
							caret
						} else {
							next_pos
						}
					},
					_ => todo!(), 
				};

				if let Some(already) = visited.get(&next_pos) {
					if *already == direction {
						// panic!("FOUND!!!");
						total += 1;
						break;
					}
				}
				visited.insert(next_pos, direction);
				caret = next_pos;
			}
		});

		// for y in 0..width {
		// 	for x in 0..height {
		// 		print!("{}", maze[x + y * width] as char);
		// 	}
		// 	println!("");
		// }
		// println!("{:?}", visited);
		// println!("");

		// maze[test] = b'.';
	}

	println!("{total}");
}
