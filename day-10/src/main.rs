fn cycle(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 1;

    for line in input {
        if line.starts_with("addx") {
            let rest = line[5..].to_string();

            let nx = rest.parse::<i32>().unwrap();

            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum += cycle * x;
            }

            x += nx;
        }

        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }
    }

    sum
}

fn part1(input: Vec<String>) {
    let sum = cycle(input.clone());

    println!("Part 1: {}", sum);
}

fn add_to_cycle(cycle: i32, x: i32) -> i32 {
    let mut nc = cycle + 1;

    let lit = if nc <= x + 2 && nc >= x { '#' } else { '.' };

    print!("{lit}");
    if nc == 40 {
        nc -= 40;
        println!()
    }

    nc
}

fn part2(input: Vec<String>) {
    let mut x = 1;
    let mut cycle = 0;

    println!("Part 2:");

    for line in input {
        cycle = add_to_cycle(cycle, x);
        if line.starts_with("addx") {
            cycle = add_to_cycle(cycle, x);

            let rest = line[5..].to_string();
            x += rest.parse::<i32>().unwrap();
        }
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
