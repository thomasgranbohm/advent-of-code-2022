enum WinState {
    WIN,
    DRAW,
    LOSS,
}

fn char_to_num(a: char) -> u32 {
    match a {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("Not allowed"),
    }
}

fn check_win(a: u32, b: u32) -> WinState {
    if a == b {
        return WinState::DRAW;
    }

    let res = (3 + (a as i32 - b as i32)) % 3;

    match res {
        0 => WinState::DRAW,
        1 => WinState::WIN,
        2 => WinState::LOSS,
        _ => panic!("Now Allowed"),
    }
}

fn get_state_score(a: WinState) -> u32 {
    match a {
        WinState::LOSS => 0,
        WinState::DRAW => 3,
        WinState::WIN => 6,
    }
}

fn num_to_state(a: u32) -> WinState {
    match a {
        1 => WinState::LOSS,
        2 => WinState::DRAW,
        3 => WinState::WIN,
        _ => panic!("Not allowed"),
    }
}

fn find_player(state: &WinState, opponent: u32) -> u32 {
    let mut choise;

    match state {
        WinState::LOSS => choise = opponent - 1,
        WinState::DRAW => choise = opponent,
        WinState::WIN => choise = opponent + 1,
    }

    if choise > 3 {
        choise -= 3
    } else if choise < 1 {
        choise += 3
    }

    choise
}

fn part1(input: Vec<[u32; 2]>) {
    let mut score: u32 = 0;

    for [opponent, player] in input {
        let state = check_win(player, opponent);

        let state_score = get_state_score(state);

        score += player;
        score += state_score;
    }

    println!("Part 1: {}", score);
}

fn part2(input: Vec<[u32; 2]>) {
    let mut score: u32 = 0;

    for [opponent, end_state] in input {
        let state = num_to_state(end_state);
        let player = find_player(&state, opponent);
        let state_score = get_state_score(state);

        score += player + state_score;
    }

    println!("Part 2: {}", score);
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    let mut matches: Vec<[u32; 2]> = Vec::new();

    for line in input {
        let mut a: [u32; 2] = [0; 2];

        for c in line.chars() {
            if c == ' ' {
                continue;
            }

            let num = char_to_num(c);

            if a[0] == 0 {
                a[0] = num;
            } else {
                a[1] = num;
            }
        }

        matches.push(a);
    }

    part1(matches.clone());
    part2(matches.clone());
}
