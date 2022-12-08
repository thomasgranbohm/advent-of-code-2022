fn find_unique_pattern(s: String, l: usize) -> i32 {
    let mut queue: Vec<char> = Vec::new();
    let mut i = 0;

    for c in s.chars() {
        if queue.len() == l {
            break;
        }

        if queue.contains(&c) {
            let index = queue.iter().position(|a| a == &c).unwrap();

            queue = queue[index + 1..].to_vec();
        }

        i += 1;

        queue.push(c);
    }

    i
}

fn part1(input: Vec<String>) {
    for line in input {
        let i = find_unique_pattern(line, 4);

        println!("Part 1: {i}");
    }
}

fn part2(input: Vec<String>) {
    for line in input {
        let i = find_unique_pattern(line, 14);

        println!("Part 1: {i}");
    }
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    part1(input.clone());
    part2(input.clone());
}
