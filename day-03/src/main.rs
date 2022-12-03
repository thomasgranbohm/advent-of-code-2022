fn char_to_num(c: char) -> i32 {
    if c.is_ascii() {
        let num: u8 = if c.is_uppercase() {
            (c as u8) - 65 + 27
        } else {
            (c as u8) - 96
        };

        return num as i32;
    }

    panic!("Not ascii char");
}

fn part1(input: Vec<String>) {
    let mut sum = 0;

    for line in input {
        let first = &line[0..line.len() / 2];
        let second = &line[line.len() / 2..];
        let mut dup: char = ' ';

        for c in second.chars() {
            if first.contains(c) {
                dup = c;
                break;
            }
        }

        sum += char_to_num(dup);
    }

    println!("Part 1: {}", sum);
}

fn part2(input: Vec<String>) {
    let mut sum = 0;

    for i in 0..input.len() / 3 {
        let first = &input[(i * 3)];
        let second = &input[(i * 3) + 1];
        let third = &input[(i * 3) + 2];

        let mut dup: char = ' ';

        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                dup = c;
                break;
            }
        }

        sum += char_to_num(dup);
    }

    println!("Part 2: {}", sum);
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    part1(input.clone());
    part2(input.clone());
}
