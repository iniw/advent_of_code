use std::fs::File;
use std::io::{self, BufRead};

struct Cmd {
	kind: String,
	val: i32,
}

fn main() {
	let file = File::open("input.txt").expect("something went wrong!");

	let input: Vec<Cmd> = io::BufReader::new(file)
		.lines()
		.map(|x| {
			let value = x.unwrap();
			let split: Vec<_> = value.split_whitespace().collect();
			Cmd {
				kind: split.first().unwrap().to_string(),
				val: split.last().unwrap().parse().unwrap(),
			}
		})
		.collect();

	let mut final_pos = 0;
	let mut final_depth = 0;
	let mut aim = 0;

	for cmd in input {
		match cmd.kind.as_str() {
			"forward" => {
				final_pos += cmd.val;
				final_depth += aim * cmd.val
			}
			"up" => aim -= cmd.val,
			"down" => aim += cmd.val,
			_ => unreachable!(),
		}
	}

	println!("result: {}", final_pos * final_depth);
}