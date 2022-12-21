use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    character: u8,
    parent: usize,
}

fn gen_graph(input: Vec<String>) -> (Vec<Node>, Vec<Vec<usize>>, usize) {
    let mut graph: Vec<Node> = Vec::new();
    let mut row_length = 0;

    for line in input {
        if row_length == 0 {
            row_length = line.len();
        };

        for c in line.chars() {
            let cc = c as usize;

            if c == 'S' {
                graph.push(Node {
                    character: 0,
                    parent: 0,
                })
            } else if c == 'E' {
                graph.push(Node {
                    character: 27,
                    parent: 0,
                })
            } else {
                graph.push(Node {
                    character: (cc - 96).try_into().unwrap(),
                    parent: 0,
                });
            }
        }
    }

    let mut connections: Vec<Vec<usize>> = Vec::new();
    let mut end_index = 0;

    for (index, curr) in graph.iter().enumerate() {
        let mut connection_row: Vec<usize> = Vec::new();

        if curr.character == 27 {
            end_index = index;
        }

        let column_index = index % row_length;
        let row_index = index / row_length;

        if (column_index >= 1) && can_move(curr.clone(), graph[index - 1]) {
            connection_row.push(index - 1);
        }
        if (column_index < row_length - 1) && can_move(curr.clone(), graph[index + 1]) {
            connection_row.push(index + 1);
        }
        if (row_index >= 1) && can_move(curr.clone(), graph[index - row_length]) {
            connection_row.push(index - row_length);
        }
        if (row_index < (graph.len() / row_length) - 1)
            && can_move(curr.clone(), graph[index + row_length])
        {
            connection_row.push(index + row_length);
        }

        connections.push(connection_row);
    }

    (graph, connections, end_index)
}

fn bfs(mut graph: Vec<Node>, connections: Vec<Vec<usize>>, end_index: usize, end_value: u8) -> u32 {
    let mut check_queue: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = graph.iter().map(|_| false).collect();

    visited[end_index] = true;

    check_queue.push_back(end_index);

    while !check_queue.is_empty() {
        let index_to_check = check_queue.pop_front().unwrap();

        if graph[index_to_check].character == end_value {
            let mut amount = 0;
            let mut qs: VecDeque<usize> = VecDeque::new();
            qs.push_back(index_to_check);

            while !qs.is_empty() {
                let inner_check_index = qs.pop_front().unwrap();

                if inner_check_index == end_index {
                    break;
                }

                qs.push_back(graph[inner_check_index].parent);
                amount += 1;
            }

            return amount;
        }

        for neighbour in connections[index_to_check].clone() {
            if !visited[neighbour] {
                visited[neighbour] = true;

                graph[neighbour].parent = index_to_check;

                check_queue.push_back(neighbour);
            }
        }
    }

    0
}

fn can_move(a: Node, b: Node) -> bool {
    b.character + 1 >= a.character
}

fn part1(input: Vec<String>) {
    let (graph, connections, end_index) = gen_graph(input.clone());
    let steps = bfs(graph, connections.clone(), end_index, 0);

    println!("Part 1: {}", steps);
}

fn part2(input: Vec<String>) {
    let (graph, connections, end_index) = gen_graph(input.clone());
    let steps = bfs(graph, connections.clone(), end_index, 1);

    println!("Part 1: {}", steps);
}

fn main() {
    let input: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect();

    part1(input.clone());
    part2(input.clone());
}
