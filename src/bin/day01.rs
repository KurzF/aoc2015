fn main() {
	let input = include_str!("../../input/day01.txt");
	let part_one = destination_floor(input);

	let part_two = first_basement(input).unwrap() + 1;
	println!("Day 01 part one: {}, part two: {}", part_one, part_two);
}

fn destination_floor(input: &str) -> i32 {
	input.chars().map(|c| match c {
		'(' => 1,
		')' => -1,
		_ => 0
	}).sum()
}

fn first_basement(input: &str) -> Option<usize> {
	let mut floor = 0;
	for (i, c) in input.char_indices() {
		floor += match c {
			'(' => 1,
			')' => -1,
			_ => 0
		};

		if floor < 0 {
			return Some(i);
		}
	}
	None
}

#[cfg(test)]
mod test {
    use crate::{destination_floor, first_basement};

	#[test]
	fn examples_destination() {
		let t1 = "(())";
		let t2 = "()()";
		assert_eq!(0, destination_floor(t1));
		assert_eq!(0, destination_floor(t2));

		let t3 = "(((";
		let t4 = "(()(()(";
		let t5 = "))(((((";
		assert_eq!(3, destination_floor(t3));
		assert_eq!(3, destination_floor(t4));
		assert_eq!(3, destination_floor(t5));

		let t6 = "())";
		let t7 = "))(";
		assert_eq!(-1, destination_floor(t6));
		assert_eq!(-1, destination_floor(t7));


		let t8 = ")))";
		let t9 = ")())())";
		assert_eq!(-3, destination_floor(t8));
		assert_eq!(-3, destination_floor(t9));
	}

	#[test]
	fn examples_basement() {
		let t1 = ")";
		assert_eq!(Some(0), first_basement(t1));
		let t2 = "()())";
		assert_eq!(Some(4), first_basement(t2));
	}
}