use fancy_regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Day8 part1: {}", solve(input.clone()));
    println!("Day8 part2: {}", solve_part2(input));
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn solve(input: String) -> i64 {
    let instructions = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut nodes = HashMap::new();
    let mut current_node = "AAA".to_string();

    for line in input.lines().skip(2) {
        let mut parts = line.split(" = ");
        let node_name = parts.next().unwrap().trim();
        let node_value = parts.next().unwrap().trim();
        nodes.insert(
            node_name.to_string(),
            Node {
                left: node_value.split(", ").nth(0).unwrap().replace("(", ""),
                right: node_value.split(", ").nth(1).unwrap().replace(")", ""),
            },
        );
    }

    let mut step_count = 0;
    let instruction_count = instructions.len();
    let mut instruction_index = 0;
    loop {
        let node = nodes.get(&current_node).unwrap();
        if instructions[instruction_index] == 'L' {
            current_node = node.left.clone();
        } else {
            current_node = node.right.clone();
        }
        step_count += 1;
        instruction_index += 1;
        if instruction_index == instruction_count {
            instruction_index = 0;
        }
        if current_node == "ZZZ" {
            break;
        }
    }

    step_count
}

fn solve_part2(input: String) -> usize {
    let re = Regex::new(r"..A").unwrap();
    let instructions = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut nodes = HashMap::new();
    let mut current_nodes = Vec::new();

    for line in input.lines().skip(2) {
        let mut parts = line.split(" = ");
        let node_name = parts.next().unwrap().trim();
        let node_value = parts.next().unwrap().trim();
        nodes.insert(
            node_name.to_string(),
            Node {
                left: node_value.split(", ").nth(0).unwrap().replace("(", ""),
                right: node_value.split(", ").nth(1).unwrap().replace(")", ""),
            },
        );

        if re.is_match(node_name).unwrap() {
            current_nodes.push(node_name.to_string());
        }
    }

    let mut step_count = 0;
    let instruction_count = instructions.len();
    let mut instruction_index = 0;
    let re = Regex::new(r"..Z").unwrap();

    loop {
        for i in 0..current_nodes.len() {
            let node = nodes.get(&current_nodes[i]).unwrap();
            if instructions[instruction_index] == 'L' {
                current_nodes[i] = node.left.clone();
            } else {
                current_nodes[i] = node.right.clone();
            }
        }

        step_count += 1;
        instruction_index += 1;
        if instruction_index == instruction_count {
            instruction_index = 0;
        }

        let mut all_match = true;
        for node in &current_nodes {
            if !re.is_match(node).unwrap() {
                all_match = false;
                break;
            }
        }

        if all_match {
            return step_count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        let part1 = solve(input.to_string());
        assert_eq!(part1, 2);
    }

    #[test]
    fn test_solve_2() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        let part1 = solve(input.to_string());
        assert_eq!(part1, 6);
    }

    #[test]
    fn test_solve_part2() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let part2 = solve_part2(input.to_string());
        assert_eq!(part2, 6);
    }
}
