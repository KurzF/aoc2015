
fn main() {

	let input = include_str!("../../input/day09.txt");
	let edges = input.lines()
		.map(|l| {
			let (edge, cost) = l.split_once(" = ").unwrap();
			let nodes = edge.split_once(" to ").unwrap();
			let cost = cost.parse().unwrap();
			(nodes, cost)
		}).collect();

	let g = Graph::new(edges);


	let mut to_visit = g.nodes.clone();
	let mut min_cost = u32::MAX;
	let mut max_cost = 0;
	let find_extremum = |permutation: &[&str]| {
		let cost = permutation.windows(2).map(|e| {
			g.get_cost(e[0], e[1])
		}).sum();

		min_cost = std::cmp::min(min_cost, cost);
		max_cost = std::cmp::max(max_cost, cost);
	};
	permutation(&mut to_visit, find_extremum);

	println!("Day 09 part one: {}, part two {}", min_cost, max_cost);
}

// Heap's algorithm
fn permutation<T, F>(elements: &mut [T], mut callback: F)
where F: FnMut(&[T])
{
	let mut c = vec![0; elements.len()];

	callback(elements);

	let mut i = 1;
	while i < elements.len() {
		if c[i] < i {
			if i % 2 == 0 {
				elements.swap(0, i);
			} else {
				elements.swap(c[i], i);
			}
			callback(elements);
			c[i] += 1;
			i = 1
		} else {
			c[i] = 0;
			i += 1;
		}
	}
}

#[derive(Debug, Clone)]
struct Graph<'a> {
	nodes: Vec<&'a str>,
	edges: Vec<((&'a str, &'a str), u32)>
}

impl<'a> Graph<'a> {
	pub fn new(edges: Vec<((&'a str, &'a str), u32)>) -> Self {
		let mut nodes = Vec::new();
		for ((node1, node2), _) in  edges.iter() {
			if !nodes.contains(node1) {
				nodes.push(node1);
			}
			if !nodes.contains(node2) {
				nodes.push(node2);
			}
		}
		Self { nodes, edges }
	}

	pub fn get_cost(&self, node1: &str, node2: &str) -> u32 {
		self.edges.iter().find(|((n1, n2), _)| (*n1 == node1 && *n2 == node2) || (*n1 == node2 && *n2 == node1)).unwrap().1
	} 
}

#[cfg(test)]
mod test {
    use std::u32;

    use crate::{permutation, Graph};


	#[test]
	fn example() {
		let input = include_str!("../../input/day09_test.txt");
		let edges = input.lines()
			.map(|l| {
				let (edge, cost) = l.split_once(" = ").unwrap();
				let nodes = edge.split_once(" to ").unwrap();
				let cost = cost.parse().unwrap();
				(nodes, cost)
			}).collect();

		let g = Graph::new(edges);

		println!("{g:?}");
		assert_eq!(3, g.nodes.len());


		let mut to_visit = g.nodes.clone();
		let mut min_cost = u32::MAX;
		let mut max_cost = 0;
		let find_extremum = |permutation: &[&str]| {
			let cost = permutation.windows(2).map(|e| {
				g.get_cost(e[0], e[1])
			}).sum();

			min_cost = std::cmp::min(min_cost, cost);
			max_cost = std::cmp::max(max_cost, cost);
		};
		permutation(&mut to_visit, find_extremum);

		assert_eq!(605, min_cost);
		assert_eq!(982, max_cost);
	}
}