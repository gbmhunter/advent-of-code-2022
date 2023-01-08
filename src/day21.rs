use std::{fs, collections::HashMap};

use indoc::indoc;
use regex::Regex;

#[derive(Debug)]
enum Node {
    number(i64),
    // First monkey, operation, second monkey 
    maths(String, String, String),

}

pub fn run() {
    println!("day21");
    let use_example = false;

    let input: String;
    if use_example {
        input = indoc! {"
            root: pppw + sjmn
            dbpl: 5
            cczh: sllz + lgvd
            zczc: 2
            ptdq: humn - dvpt
            dvpt: 3
            lfqf: 4
            humn: 5
            ljgn: 2
            sjmn: drzm * dbpl
            sllz: 4
            pppw: cczh / lfqf
            lgvd: ljgn * ptdq
            drzm: hmdt - zczc
            hmdt: 32
        "}.to_string();
    } else {
        input = fs::read_to_string("src/day21.txt").unwrap();
    }

    let single_number_regex = Regex::new(r"^([a-z]{4}): ([0-9]*)$").unwrap();
    let maths_regex = Regex::new(r"^([a-z]{4}): ([a-z]{4}) (.) ([a-z]{4})$").unwrap();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in input.lines() {
        if let Some(capture) = single_number_regex.captures(line) {
            let monkey = capture.get(1).unwrap().as_str();
            let number = capture.get(2).unwrap().as_str();
            println!("monkey = {}, number = {}", monkey, number);
            nodes.insert(monkey.to_string(), Node::number(number.parse::<i64>().unwrap()));
        } else if let Some(capture) = maths_regex.captures(line) {
            let monkey = capture.get(1).unwrap().as_str();
            let child_monkey_1 = capture.get(2).unwrap().as_str();
            let operation = capture.get(3).unwrap().as_str();
            let child_monkey_2 = capture.get(4).unwrap().as_str();
            nodes.insert(
                monkey.to_string(),
                Node::maths(
                    child_monkey_1.to_string(),
                    operation.to_string(),
                    child_monkey_2.to_string(),
            ));
            println!("monkey = {}, child_monkey_1 = {}, operation = {}, child_monkey_2 = {}", monkey, child_monkey_1, operation, child_monkey_2)
        }
    }
    println!("nodes = {:?}", nodes);

    let solution = solve(&nodes, "root");
    println!("part 1: root yells = {}", solution);
    if use_example {
        assert!(solution == 152)
    } else {
        assert!(solution == 160274622817992)
    }
}

fn solve(nodes: &HashMap<String, Node>, node_to_solve: &str) -> i64 {
    // Get node
    match &nodes[node_to_solve] {
        Node::number(number) => return *number,
        Node::maths(child_monkey_1, operation, child_monkey_2) => {
            let c1_val = solve(nodes, &child_monkey_1);
            let c2_val = solve(nodes, &child_monkey_2);
            match operation.as_str() {
                "+" => c1_val + c2_val,
                "-" => c1_val - c2_val,
                "*" => c1_val * c2_val,
                "/" => c1_val / c2_val,
                _ => panic!("hshsh")
            }
        }
    }
}