
fn main() {
	let input = include_str!()
}

fn part_one(input: &str) -> u32 {
	let mut answer = 0;
	loop {
		let hash = md5::compute(format!("{}{}", input, answer));
		if hash.0[0] == 0 && hash.0[1] == 0 && (hash.0[2] & 0x0f == 0) {
			break;
		}
		answer += 1;

	}
	
	answer
}
