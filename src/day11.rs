use std::fs;

use evalexpr::*;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: String,
    test_divisible_by: i32,
    test_true_monkey_num: i32,
    test_false_monkey_num: i32,
}

pub fn run() {
    println!("day11");
    let contents = fs::read_to_string("src/day11.txt").unwrap();

    let mut curr_line = 0;

    let lines: Vec<&str> = contents.lines().collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    while curr_line < lines.len() {
        // Parse monkey
        println!("Parsing monkey {}", monkeys.len());

        let starting_line = &lines[curr_line + 1][17..];
        println!("starting={}", starting_line);
        let pieces: Vec<&str> = starting_line.split(",").collect();
        let mut items: Vec<i32> = Vec::new();
        for piece in pieces {
            items.push(piece.trim().parse::<i32>().unwrap());
        }

        // Operation

        // Don't include the "new =", as we want a valid "expression" for the parsing library 
        let operation = lines[curr_line + 2][19..].to_string();
        println!("operation={}", operation);

        // Test
        let test_divisible_by_str = &lines[curr_line + 3][21..];
        println!("test_divisible_by_str={}", test_divisible_by_str);
        let test_divisible_by = test_divisible_by_str.parse::<i32>().unwrap();
        let test_true_monkey_num = lines[curr_line + 4][29..].parse::<i32>().unwrap();
        let test_false_monkey_num = lines[curr_line + 5][30..].parse::<i32>().unwrap();
        println!("test_true_monkey_num={}", test_true_monkey_num);
        println!("test_false_monkey_num={}", test_false_monkey_num);

        let monkey = Monkey {
            items,
            operation,
            test_divisible_by,
            test_true_monkey_num,
            test_false_monkey_num,
        };

        monkeys.push(monkey);

        // Jump to the start of the next monkey
        curr_line += 7;
    }

    println!("Parsing monkeys finished. monkeys={:#?}", monkeys);

    for round_idx in 0..1 {
        println!("Starting round {}", round_idx);
        for monkey in &monkeys {
            // Inspect items
            for item in &monkey.items {
                println!("item = {}", item);

                let num = *item as i64;
                let context = context_map! {
                    "old" => num,
                }.unwrap();
                let mut new_worry_level = eval_with_context(monkey.operation.as_str(), &context).unwrap().as_int().unwrap() as i32;
                // Divide by 3 and round down
                new_worry_level = new_worry_level / 3;
                println!("Result = {}", new_worry_level);
                // Test if divisible
                if new_worry_level % monkey.test_divisible_by == 0 {
                    println!("Test was TRUE.")
                } else {
                    println!("Test was FALSE.")
                }
            }
        }
    }
}