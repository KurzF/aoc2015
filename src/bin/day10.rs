
fn main() {

	let input = include_str!("../../input/day10.txt");
	
	let mut work = String::from(input);
	for _ in 0..40 {
		work = look_and_say(&work);
	}
	let part_one = work.len();

	for _ in 0..10 {
		work = look_and_say(&work);
	}
	let part_two = work.len();
	println!("Day 09 part one: {}, part two: {}", part_one, part_two);
}

fn look_and_say(input: &str) -> String {
	let char_count = input.chars().fold(Vec::new(), |mut acc, c| {
		match acc.last_mut() { 
			Some((count, previous_char)) if *previous_char == c => {
				*count += 1;
			},
			_ => {
				acc.push((1, c));
			}
		};
		acc
	});

	let mut result = String::new();
	for (count, ch) in char_count.iter() {
		result.push_str(&count.to_string());
		result.push(*ch);
	}

	return result;
}

#[cfg(test)]
mod test {
    use crate::look_and_say;




	#[test]
	fn single() {
		let s = String::from("1");
		let result = look_and_say(&s);
		assert_eq!("11", result);
	}
	
	#[test]
	fn double() {
		let s = String::from("11");
		let result = look_and_say(&s);
		assert_eq!("21", result);
	}

	#[test]
	fn complex() {
		let s = "111221";
		let result = look_and_say(s);
		assert_eq!("312211", result);
	}
}