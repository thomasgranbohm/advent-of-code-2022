use core::panic;
use std::{collections::HashMap, vec};

#[derive(Copy, Clone, Debug)]
struct Movement {
    x: i32,
    y: i32,
}

fn get_movement(i: String) -> Movement {
    let splitted = i.split(" ").collect::<Vec<&str>>();
    let amount = splitted[1].parse::<i32>().unwrap();

    match splitted[0] {
        "U" => Movement {
            x: 0,
            y: -1 * amount,
        },
        "D" => Movement { x: 0, y: amount },
        "L" => Movement {
            x: -1 * amount,
            y: 0,
        },
        "R" => Movement { x: amount, y: 0 },
        _ => panic!("What the fuck"),
    }
}

fn get_knot_movement(k: Movement, h: Movement) -> Movement {
    let dx = h.x - k.x;
    let dy = h.y - k.y;
    let should_move = dx.abs() > 1 || dy.abs() > 1;

    if !should_move {
        return k;
    }

    let diagonal = (dx.abs() > 1 && dy.abs() >= 1) || (dx.abs() >= 1 && dy.abs() > 1);

    if diagonal {
        return Movement {
            x: k.x + (if dx.is_negative() { -1 } else { 1 }),
            y: k.y + (if dy.is_negative() { -1 } else { 1 }),
        };
    } else {
        if dy == 0 {
            return Movement {
                x: k.x + (if dx.is_negative() { -1 } else { 1 }),
                y: k.y,
            };
        } else if dx == 0 {
            return Movement {
                x: k.x,
                y: k.y + (if dy.is_negative() { -1 } else { 1 }),
            };
        }
    }

    panic!("Movement was wrong ({}, {})", dx, dy)
}

fn move_head(h: Movement, m: Movement) -> Movement {
    let mut c = h.clone();

    if m.x == 0 {
        c.y += m.y / m.y.abs();
    }
    if m.y == 0 {
        c.x += m.x / m.x.abs();
    }

    c
}

fn part1(input: Vec<String>) {
    let mut grid: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut head = Movement { x: 0, y: 0 };
    let mut tail = head.clone();

    for line in input {
        let movement = get_movement(line);

        for _ in 0..movement.x.abs().max(movement.y.abs()) {
            head = move_head(head, movement);
            tail = get_knot_movement(tail, head);

            if grid.contains_key(&tail.y) {
                let v = grid.get_mut(&tail.y).unwrap();
                if !v.contains(&tail.x) {
                    v.push(tail.x)
                }
            } else {
                grid.insert(tail.y, vec![tail.x]);
            }
        }
    }

    let sum: u32 = grid.values().map(|xs| xs.len() as u32).sum::<u32>();

    println!("Part 1: {sum}");
}

fn part2(input: Vec<String>) {
    let mut grid: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut knots = [Movement { x: 0, y: 0 }; 10];

    for line in input {
        let movement = get_movement(line);

        for _ in 0..movement.x.abs().max(movement.y.abs()) {
            // Move head
            knots[0] = move_head(knots[0], movement);

            for i in 1..knots.len() {
                knots[i] = get_knot_movement(knots[i], knots[i - 1].clone());
            }

            let last_knot = knots[9].clone();
            if grid.contains_key(&last_knot.y) {
                let v = grid.get_mut(&last_knot.y).unwrap();
                if !v.contains(&last_knot.x) {
                    v.push(last_knot.x)
                }
            } else {
                grid.insert(last_knot.y, vec![last_knot.x]);
            }
        }
    }

    let sum: u32 = grid.values().map(|xs| xs.len() as u32).sum::<u32>();

    println!("Part 2: {sum}");
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    part1(input.clone());
    part2(input.clone());
}
