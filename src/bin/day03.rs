use std::collections::HashSet;


fn main() {
	let input = include_str!("../../input/day03.txt");
	let part_one = part_one(input);
	let part_two = part_two(input);
	println!("Day 03 part one: {}, part_two: {}", part_one, part_two);
}

fn part_one(input: &str) -> usize {
	let mut position = [0, 0];
	let mut visited = HashSet::<[i32; 2]>::new();
	visited.insert(position);

	for c in input.chars() {
		match c {
			'^' => position[1] += 1,
			'v' => position[1] -= 1,
			'<' => position[0] -= 1,
			'>' => position[0] += 1,
			_ => {}
		};

		visited.insert(position);
	}

	visited.len()
}

fn part_two(input: &str) -> usize {
	let mut santa_position = [0, 0];
	let mut robot_position = [0, 0];

	let mut visited = HashSet::<[i32; 2]>::new();
	visited.insert(santa_position);

	for (i, c) in input.char_indices() {
		if i % 2 == 0 {
			match c {
				'^' => santa_position[1] += 1,
				'v' => santa_position[1] -= 1,
				'<' => santa_position[0] -= 1,
				'>' => santa_position[0] += 1,
				_ => {}
			};

			visited.insert(santa_position);
		} else {
			match c {
				'^' => robot_position[1] += 1,
				'v' => robot_position[1] -= 1,
				'<' => robot_position[0] -= 1,
				'>' => robot_position[0] += 1,
				_ => {}
			};

			visited.insert(robot_position);
		}
	}

	visited.len()
}