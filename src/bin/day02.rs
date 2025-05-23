fn main() {
	let input = include_str!("../../input/day02.txt");
	let part_one: u32 = parse_dimension(input).map(total_wrapping).sum();
	let part_two: u32 = parse_dimension(input).map(ribbon_length).sum();

	println!("Day 02 part one: {}, part two: {}", part_one, part_two);
}

fn parse_dimension<'a>(input: &'a str) -> impl Iterator<Item = [u32; 3]> + use<'a> {
	input.lines().map(|l| {
		let mut dimensions: Vec<u32> = l.split('x').map(|dim| dim.parse::<u32>().unwrap()).collect();
		dimensions.sort_unstable();
		let l = dimensions[0];
		let w = dimensions[1];
		let h = dimensions[2];

		[l, w, h]
	})
}

fn total_wrapping(dim: [u32; 3]) -> u32 {
	let area = 2 * (dim[0] * dim[1] + dim[1] * dim[2] + dim[2] * dim[0]);
	let slack = dim[0] * dim[1];

	area + slack
}

fn ribbon_length(dim: [u32; 3]) -> u32 {
	let wrap = 2 * (dim[0] + dim[1]);
	let bow = dim[0] * dim[1] * dim[2];

	wrap + bow
}

#[cfg(test)]
mod test {

}