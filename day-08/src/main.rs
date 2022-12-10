fn gen_grid(input: Vec<String>) -> Vec<Vec<i8>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i8>().unwrap())
                .collect()
        })
        .collect()
}

fn cmp_arr_to_num(v: &[i8], a: i8) -> bool {
    v.to_vec().iter().max().unwrap() < &a
}

fn part1(input: Vec<String>) {
    let grid: Vec<Vec<i8>> = gen_grid(input);

    let size = grid.len();

    let mut visible = 0;

    for y in 1..size - 1 {
        for x in 1..size - 1 {
            let curr = grid[y][x];
            let c: Vec<i8> = grid.iter().map(|row| row[x]).collect();

            let biggest_left = cmp_arr_to_num(&grid[y][..x], curr);
            let biggest_right = cmp_arr_to_num(&grid[y][x + 1..], curr);
            let biggest_top = cmp_arr_to_num(&c[0..y], curr);
            let biggest_bottom = cmp_arr_to_num(&c[y + 1..], curr);

            if biggest_left || biggest_right || biggest_top || biggest_bottom {
                visible += 1;
            }
        }
    }

    visible += (4 * size) - 4;

    println!("Part 1: {visible}");
}

fn index_until(v: &[i8], c: i8, d: usize, rev: bool) -> usize {
    let mut b = d;

    if rev == true {
        for (i, &a) in v.iter().enumerate().rev() {
            if a >= c {
                b = i;
                break;
            }
        }
    } else {
        for (i, &a) in v.iter().enumerate() {
            if a >= c {
                b = i;
                break;
            }
        }
    }

    b
}

fn part2(input: Vec<String>) {
    let grid: Vec<Vec<i8>> = gen_grid(input);

    let size = grid.len();

    let mut scenic_score = 1;

    for y in 1..size - 1 {
        for x in 1..size - 1 {
            let curr = grid[y][x];
            let c: Vec<i8> = grid.iter().map(|row| row[x]).collect();

            let mut local_scenic = 1;

            let left = x - index_until(&grid[y][..x], curr, 0, true);
            let right = 1 + index_until(&grid[y][x + 1..], curr, grid[y][x + 1..].len() - 1, false);
            let up = y - index_until(&c[..y], curr, 0, true);
            let down = 1 + index_until(&c[y + 1..], curr, c[y + 1..].len() - 1, false);

            local_scenic *= left * right * up * down;

            if local_scenic >= scenic_score {
                scenic_score = local_scenic;
            }
        }
    }

    println!("Part 2: {scenic_score}");
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    part1(input.clone());
    part2(input.clone());
}
