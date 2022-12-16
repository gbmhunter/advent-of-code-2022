use std::fs;

use evalexpr::*;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Node,
    test_divisible_by: i64,
    test_true_monkey_num: i64,
    test_false_monkey_num: i64,
    num_items_inspected: i64,
}

pub fn run() {
    println!("day11");
    let contents = fs::read_to_string("src/day11.txt").unwrap();

    let mut curr_line = 0;

    let lines: Vec<&str> = contents.lines().collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    while curr_line < lines.len() {
        // Parse monkey
        let starting_line = &lines[curr_line + 1][17..];
        let pieces: Vec<&str> = starting_line.split(",").collect();
        let mut items: Vec<i64> = Vec::new();
        for piece in pieces {
            items.push(piece.trim().parse::<i64>().unwrap());
        }

        // Operation
        // Don't include the "new =", as we want a valid "expression" for the parsing library 
        let operation_str = lines[curr_line + 2][19..].to_string();
        // Pre compile to make it faster later
        let operation = build_operator_tree(&operation_str).unwrap();

        // Test
        let test_divisible_by_str = &lines[curr_line + 3][21..];
        let test_divisible_by = test_divisible_by_str.parse::<i64>().unwrap();
        let test_true_monkey_num = lines[curr_line + 4][29..].parse::<i64>().unwrap();
        let test_false_monkey_num = lines[curr_line + 5][30..].parse::<i64>().unwrap();

        // Start with no items inspected, this gets incremented as we go
        let num_items_inspected = 0;

        let monkey = Monkey {
            items,
            operation,
            test_divisible_by,
            test_true_monkey_num,
            test_false_monkey_num,
            num_items_inspected,
        };

        monkeys.push(monkey);

        // Jump to the start of the next monkey
        curr_line += 7;
    }

    part1(monkeys.clone());
    part2(monkeys.clone());
}

fn part1(mut monkeys: Vec<Monkey>) {

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // This is get around the issue of mutably borrowing more than one monkey at the same time
            let monkey = monkeys[i].clone();
            // Inspect items

            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items.remove(0);
                let num = item as i64;
                let context = context_map! {
                    "old" => num,
                }.unwrap();
                let mut new_worry_level = monkey.operation.eval_with_context(&context).unwrap().as_int().unwrap() as i64;
                // Divide by 3 and round down
                new_worry_level = new_worry_level / 3;
                // Test if divisible
                if new_worry_level % monkey.test_divisible_by == 0 {
                    monkeys[monkey.test_true_monkey_num as usize].items.push(new_worry_level);
                } else {
                    monkeys[monkey.test_false_monkey_num as usize].items.push(new_worry_level);
                }
                // Finished inpecting an item, keep track of the total items inspected per monkey
                monkeys[i].num_items_inspected += 1;
            }
        }
    }

    let mut num_items_inspected = monkeys.iter().map(|x| x.num_items_inspected).collect::<Vec<_>>();
    num_items_inspected.sort();
    num_items_inspected.reverse();
    let monkey_business = num_items_inspected[0] * num_items_inspected[1];
    println!("part 1: monkey business = {:?}", monkey_business);
    
}

fn part2(mut monkeys: Vec<Monkey>) {

    let mut product_of_all_divisors = 1;
    for monkey in &monkeys {
        product_of_all_divisors *= monkey.test_divisible_by;
    }

    for round_idx in 0..10000 {
        if round_idx % 1000 == 0 {
            println!("Starting round {}", round_idx);
        }
        for i in 0..monkeys.len() {
            // This is get around the issue of mutably borrowing more than one monkey at the same time
            let monkey = monkeys[i].clone();
            // Inspect items
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items.remove(0);

                let num = item as i64;
                let context = context_map! {
                    "old" => num,
                }.unwrap();
                let mut new_worry_level = monkey.operation.eval_with_context(&context).unwrap().as_int().unwrap() as i64;
                // Divide by 3 and round down
                // new_worry_level = new_worry_level / 3;

                // Keep worry level from getting too big
                new_worry_level = new_worry_level % product_of_all_divisors;

                // Test if divisible
                if new_worry_level % monkey.test_divisible_by == 0 {
                    monkeys[monkey.test_true_monkey_num as usize].items.push(new_worry_level);
                } else {
                    monkeys[monkey.test_false_monkey_num as usize].items.push(new_worry_level);
                }
                // Finished inpecting an item, keep track of the total items inspected per monkey
                monkeys[i].num_items_inspected += 1;
            }
        }
    }

    let mut num_items_inspected = monkeys.iter().map(|x| x.num_items_inspected).collect::<Vec<_>>();
    num_items_inspected.sort();
    num_items_inspected.reverse();
    let monkey_business = num_items_inspected[0] * num_items_inspected[1];
    println!("part 2: monkey business = {:?}", monkey_business);
    
}