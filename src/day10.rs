use std::fs;

pub fn run() {
    println!("day10");
    let contents = fs::read_to_string("src/day10.txt").unwrap();
    let mut lines = contents.lines();

    let mut cycle_count = 0;
    while(true) {

        // Time to grab another instruction
        let line = lines.next().unwrap();
        println!("{}", line);

        if line == "noop" {
            println!("Found noop");
        } else if line.starts_with("addx") {
            println!("Found addx command");
            let pieces: Vec<&str> = line.split(" ").collect();
            let add_amount = pieces[1].parse::<i32>().unwrap();
            println!("add_amount={}", add_amount);
        }
        cycle_count += 1;
    }
}