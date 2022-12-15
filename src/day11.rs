use std::fs;

struct Monkey {
    items: Vec<i32>,
    operation: String,
    test: i32,
}

pub fn run() {
    println!("day11");
    let contents = fs::read_to_string("src/day11.txt").unwrap();

    let mut curr_line = 0;

    let lines: Vec<&str> = contents.lines().collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    while curr_line < lines.len() {
        // Parse monkey
        let mut monkey = Monkey();

        let starting_line = &lines[curr_line + 1][17..];
        println!("starting={}", starting_line);
        let pieces: Vec<&str> = starting_line.split(",").collect();
        for piece in pieces {
            piece.parse::()
        }

        // Jump to the start of the next monkey
        curr_line += 7;
    }
}