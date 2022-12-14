use std::fs;

pub fn run() {
    println!("day10");
    let contents = fs::read_to_string("src/day10.txt").unwrap();
    let mut lines = contents.lines();

    let mut cycle_count = 0;
    let mut curr_command_remaining_cycles = 0;

    while(true) {
        println!("Starting cycle. cycle_count={}", cycle_count);
       

        if curr_command_remaining_cycles == 0 {
            println!("Time to read in a new command!");
             // Time to grab another instruction
            let mut line = "";
            match lines.next() {
                None => {
                    println!("Run out of commands.");
                    break;
                },
                Some(i) => line = i,
            }
            println!("{}", line);
            if line == "noop" {
                println!("Found noop");
                curr_command_remaining_cycles = 1;
            } else if line.starts_with("addx") {
                println!("Found addx command");
                let pieces: Vec<&str> = line.split(" ").collect();
                let add_amount = pieces[1].parse::<i32>().unwrap();
                println!("add_amount={}", add_amount);
                curr_command_remaining_cycles = 2;
            }
        }

        if curr_command_remaining_cycles > 0 {
            curr_command_remaining_cycles -= 1;
        }
        cycle_count += 1;
    }
}