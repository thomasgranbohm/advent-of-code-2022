fn part1(pairs: Vec<[(i32, i32); 2]>) {
    let sum: i32 = pairs
        .iter()
        .map(|[(s1, e1), (s2, e2)]| {
            if (s1 - s2 >= 0 && e2 - e1 >= 0) || (s2 - s1 >= 0 && e1 - e2 >= 0) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("Part 1: {sum}");
}

fn part2(pairs: Vec<[(i32, i32); 2]>) {
    let sum: i32 = pairs
        .iter()
        .map(|[(s1, e1), (s2, e2)]| if s1 > e2 || s2 > e1 { 0 } else { 1 })
        .sum();

    println!("Part 2: {sum}");
}

fn main() {
    let input: Vec<[(i32, i32); 2]> = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let i = line.find(",").unwrap();

            [&line[0..i], &line[i + 1..]]
        })
        .map(|pair| {
            pair.map(|a| {
                let i: Vec<&str> = a.split("-").collect();

                (i[0].parse::<i32>().unwrap(), i[1].parse::<i32>().unwrap())
            })
        })
        .collect();

    part1(input.clone());
    part2(input.clone());
}
