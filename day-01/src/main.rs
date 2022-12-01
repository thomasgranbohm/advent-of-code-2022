fn calc_calories(input: Vec<String>) -> Vec<i32> {
	let mut index = 0;
	let mut amounts: Vec<i32> = Vec::new();

	for line in input {
		if line.len() == 0 {
			index += 1;
			continue;
		}

		let value = line.parse::<i32>().unwrap();

		if amounts.get(index) != None {
			amounts[index] += value;
		} else {
			amounts.push(value);
		}
	}

	amounts.sort();

	amounts
}

fn part1(input: Vec<String>) {
	let calories = calc_calories(input);

	println!("Part 1: {}", calories[calories.len() - 1]);
}

fn part2(input: Vec<String>) {
	let calories = calc_calories(input);

	let length = calories.len();
	let top_three = &calories[(length - 3)..(length)];

	println!("Part 2: {}", top_three.iter().sum::<i32>());
}

fn main() {
	let input: Vec<String> = include_str!("./input.txt")
		.lines()
		.map(|a| a.to_string())
		.collect();

	part1(input.clone());
	part2(input.clone());
}
