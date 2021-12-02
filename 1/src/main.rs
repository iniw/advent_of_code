use std::fs::File;
use std::io::{self, BufRead};

fn main() {
	let file = File::open("input.txt").expect("something went wrong!");

	let input: Vec<u32> = io::BufReader::new(file)
		.lines()
		.map(|x| x.unwrap().parse().unwrap())
		.collect();

	// part one
	let mut iter = input.iter().peekable();
	let mut counter = 0;

	while let Some(x) = iter.next() {
		if let Some(next) = iter.peek() {
			if x < *next {
				counter += 1;
			}
		}
	}

	println!("part one: {}", counter);

	// part two
	let mut iter = input.windows(3).peekable();
	let mut counter = 0;

	while let Some(x) = iter.next() {
		if let Some(next) = iter.peek() {
			if x.iter().sum::<u32>() < next.iter().sum() {
				counter += 1;
			}
		}
	}

	println!("part two: {}", counter)
}