fn get_dir_size(lines: Vec<String>) -> (i32, i32, usize, Vec<i32>) {
    let mut size = 0;
    let mut q_size = 0;
    let mut should_skip = 0;
    let mut acc = Vec::new();

    for line_index in 0..lines.len() {
        if should_skip > 0 {
            should_skip -= 1;
            continue;
        }

        let line = &lines[line_index];

        if line.starts_with("$ ") {
            let command_line: Vec<&str> = line[2..].split(" ").collect();

            let command = command_line[0];

            if command == "cd" {
                if command_line[1] == ".." {
                    acc.push(size);

                    return (
                        size,
                        if size <= 100000 {
                            q_size + size
                        } else {
                            q_size
                        },
                        line_index,
                        acc,
                    );
                }

                if command_line[1] != ".." {
                    let nl: Vec<String> = lines[line_index + 1..]
                        .iter()
                        .map(|a| a.to_owned())
                        .collect();

                    let r = get_dir_size(nl.clone());

                    size += r.0;
                    q_size += r.1;

                    should_skip = r.2 + 1;

                    if r.3.len() > 0 {
                        r.3.iter().for_each(|&a| acc.push(a));
                    }
                }
            }
        } else {
            let id: Vec<&str> = line.split(" ").collect();
            size += id[0].parse::<i32>().unwrap_or(0);
        }
    }

    acc.push(size);

    (
        size,
        if size <= 100000 {
            q_size + size
        } else {
            q_size
        },
        lines.len() - 1,
        acc,
    )
}

fn part1(input: Vec<String>) {
    let (_, q_size, _, _) = get_dir_size(input);

    println!("Part 1: {q_size}")
}

fn part2(input: Vec<String>) {
    let (total_size, _, _, acc) = get_dir_size(input);

    let needed = &total_size - (70000000 - 30000000);

    let minimum = acc.iter().filter(|&a| a >= &needed).min().unwrap();

    println!("Part 2: {minimum}")
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    part1(input.clone());
    part2(input.clone());
}
