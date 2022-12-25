use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Monkey {
    operation_string: Vec<String>,
    test_number: u128,
    truthy_monkey: usize,
    falsy_monkey: usize,
    inspection_amount: u128,
}

impl Monkey {
    fn operate(&mut self, item: u128) -> u128 {
        let first = &self.operation_string[0];
        let operator: char = self.operation_string[1].parse::<char>().unwrap();
        let second = &self.operation_string[2];

        let mut numbers: (u128, u128) = (0, 0);

        if first == "old" {
            numbers.0 = item;
        } else {
            numbers.0 = first.parse::<u128>().unwrap();
        }

        if second == "old" {
            numbers.1 = item;
        } else {
            numbers.1 = second.parse::<u128>().unwrap();
        }

        self.inspection_amount += 1;

        match operator {
            '+' => numbers.0 + numbers.1,
            '-' => numbers.0 - numbers.1,
            '*' => numbers.0 * numbers.1,
            _ => panic!("Unknown operation"),
        }
    }
}

fn get_monkeys(input: Vec<String>) -> (Vec<Monkey>, Vec<VecDeque<u128>>) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items: Vec<VecDeque<u128>> = Vec::new();

    while monkeys.len() * 7 < input.len() {
        let lines: Vec<String> = ((monkeys.len() * 7)..(monkeys.len() * 7) + 6)
            .into_iter()
            .map(|i| input.get(i).unwrap().to_string())
            .collect();

        let (monkey, starting_items) = parse_monkey(lines);
        monkeys.push(monkey);
        items.push(starting_items);
    }

    (monkeys, items)
}

fn parse_monkey(lines: Vec<String>) -> (Monkey, VecDeque<u128>) {
    let starting_items: VecDeque<u128> = lines[1][18..]
        .split(", ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    let operation: Vec<String> = lines[2][19..].split(" ").map(|x| x.to_string()).collect();
    let test: u128 = lines[3][21..].parse().unwrap();
    let truthy = lines[4][29..].parse::<usize>().unwrap();
    let falsy = lines[5][30..].parse::<usize>().unwrap();

    (
        Monkey {
            falsy_monkey: falsy,
            operation_string: operation,
            test_number: test,
            truthy_monkey: truthy,
            inspection_amount: 0,
        },
        starting_items,
    )
}

fn part1(input: Vec<String>) {
    let (mut monkeys, mut items) = get_monkeys(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while items[i].len() > 0 {
                let item = items[i].pop_front().unwrap();
                let res = monkeys[i].operate(item) / 3;

                let m = if res % monkeys[i].test_number == 0 {
                    monkeys[i].truthy_monkey
                } else {
                    monkeys[i].falsy_monkey
                };

                items[m].push_back(res);
            }
        }
    }

    let mut biggest: Vec<u128> = monkeys
        .clone()
        .iter()
        .map(|x| x.inspection_amount)
        .collect();
    biggest.sort();
    biggest.reverse();

    println!("Part 1: {}", biggest[0] * biggest[1]);
}

fn part2(input: Vec<String>) {
    let (mut monkeys, mut items) = get_monkeys(input);

    let mut bar = 1;
    for monkey in monkeys.clone() {
        bar *= monkey.test_number;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while items[i].len() > 0 {
                let item = items[i].pop_front().unwrap();
                let res = monkeys[i].operate(item) % bar;

                let m = if res % monkeys[i].test_number == 0 {
                    monkeys[i].truthy_monkey
                } else {
                    monkeys[i].falsy_monkey
                };

                items[m].push_back(res);
            }
        }
    }

    let mut biggest: Vec<u128> = monkeys
        .clone()
        .iter()
        .map(|x| x.inspection_amount)
        .collect();
    biggest.sort();
    biggest.reverse();

    println!("Part 2: {}", biggest[0] * biggest[1]);
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();
    part1(input.clone());
    part2(input.clone());
}
