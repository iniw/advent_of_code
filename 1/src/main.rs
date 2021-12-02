use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").expect("something went wrong!");

	let input: Vec<u32> = io::BufReader::new(file)
		.lines()
		.map(|x| x.unwrap().parse().unwrap())
		.collect();

	// part one
	let result = input.windows(2).filter(|x| x.first() < x.last()).count();
	println!("part one: {}", result);

	// part two
	let result = input.windows(4).filter(|x| x.first() < x.last()).count();
	println!("part two: {}", result)
}