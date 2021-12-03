use std::fs::File;
use std::io::{self, BufRead};

// could grab this dinamically based on the size of the line but who cares
const NUM_BITS: u32 = 12;

fn part_one(input: &Vec<u32>) -> u32 {
	let mut gamma_rate = 0;
	let mut epsilon_rate = 0;

	for i in 0..NUM_BITS {
		let mut ones = 0;
		let mut zeros = 0;

		for num in input.iter() {
			let bit = *num >> i & 1;

			match bit {
				1 => ones += 1,
				0 => zeros += 1,
				_ => unreachable!(),
			}
		}

		let result = ((ones > zeros) as u32) << i;

		gamma_rate |= result;
		epsilon_rate |= result ^ (1 << i);
	}

	gamma_rate * epsilon_rate
}

fn part_two(input: &Vec<u32>) -> u32 {
	fn find_rating(input: &Vec<u32>, bit_pref: bool) -> u32 {
		let mut filtered = input.clone();

		for i in 1..=NUM_BITS {
			let mut indices: [Vec<usize>; 2] = [Vec::<usize>::new(), Vec::<usize>::new()];

			if filtered.len() == 1 {
				break;
			}

			for (index, num) in filtered.iter().enumerate().rev() {
				let bit = num >> (NUM_BITS - i) & 1;
				indices[bit as usize].push(index);
			}

			let selection = if bit_pref {
				indices.iter().min_by_key(|x| x.len()).unwrap().clone()
			} else {
				indices.iter().max_by_key(|x| x.len()).unwrap().clone()
			};

			let same = indices[1].len() == indices[0].len();

			if !same {
				for x in selection {
					filtered.remove(x);
				}
			} else {
				let selection = &indices[!bit_pref as usize];
				for x in selection {
					filtered.remove(*x);
				}
			}
		}

		filtered[0]
	}

	let o2_rating = find_rating(&input, true);

	let co2_rating = find_rating(&input, false);

	o2_rating * co2_rating
}

fn main() {
	let file = File::open("input.txt").expect("something went wrong!");

	let input: Vec<u32> = io::BufReader::new(file)
		.lines()
		.map(|x| u32::from_str_radix(x.unwrap().as_str(), 2).unwrap())
		.collect();

	println!("part one: {}", part_one(&input));
	println!("part two: {}", part_two(&input));
}