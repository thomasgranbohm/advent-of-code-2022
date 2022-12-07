fn get_board(input: Vec<String>) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = vec![Vec::new(); 9];
    let blueprints: Vec<&String> = input.iter().take_while(|a| a.contains("[")).collect();

    for blueprint in blueprints.iter().rev() {
        for (i, c) in blueprint.chars().enumerate() {
            if i % 4 != 1 || c == ' ' {
                continue;
            }

            board[(i / 4)].push(c);
        }
    }

    board
}

fn get_instructions(input: Vec<String>) -> Vec<(i8, i8, i8)> {
    let mut instructions: Vec<(i8, i8, i8)> = Vec::new();

    for instruction in input.iter().filter(|a| a.starts_with("move")) {
        let v = instruction
            .split(" ")
            .filter_map(|b| b.parse::<i8>().ok())
            .collect::<Vec<i8>>();

        instructions.push((v[0], v[1] - 1, v[2] - 1));
    }

    instructions
}

fn get_answer(board: Vec<Vec<char>>) -> String {
    board
        .iter()
        .map(|a| a.clone().pop().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn part1(input: Vec<String>) {
    let mut board = get_board(input.clone());
    let instructions = get_instructions(input.clone());

    for (amount, from, to) in instructions {
        for _ in 0..amount {
            let to_move = board[from as usize].pop().unwrap();
            board[to as usize].push(to_move);
        }
    }

    println!("Part 1: {}", get_answer(board));
}
fn part2(input: Vec<String>) {
    let mut board = get_board(input.clone());
    let instructions = get_instructions(input.clone());

    for (amount, from, to) in instructions {
        let mut movers = Vec::new();
        for _ in 0..amount {
            let to_move = board[from as usize].pop().unwrap();
            movers.push(to_move);
        }
        for to_move in movers.iter().rev() {
            board[to as usize].push(*to_move);
        }
    }

    println!("Part 2: {}", get_answer(board));
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    part1(input.clone());
    part2(input.clone());
}
