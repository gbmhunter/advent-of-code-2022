use std::fs;

pub fn run() {
    println!("day10");
    let contents = fs::read_to_string("src/day10.txt").unwrap();
    let mut lines = contents.lines();

    let mut cycle_count = 1; // Elves start their cycle count at 1
    let mut curr_command_remaining_cycles = 0;
    let mut curr_command_name = "";
    let mut curr_command_reg_value = 0;
    let mut x_reg_value = 1;

    let mut sum_of_signal_strengths = 0;
    let signal_strength_start_count = 20;
    let signal_strength_period = 40; // Every 40 cycles after the 20th

    while(true) {
        println!("Starting cycle. cycle_count={}, curr_command_name={}, curr_command_reg_value={}, curr_command_remaining_cycles={}",
            cycle_count, curr_command_name, curr_command_reg_value, curr_command_remaining_cycles);
       

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
                curr_command_name = line;
                curr_command_remaining_cycles = 1;
            } else if line.starts_with("addx") {
                println!("Found addx command");
                let pieces: Vec<&str> = line.split(" ").collect();
                let add_amount = pieces[1].parse::<i32>().unwrap();
                println!("add_amount={}", add_amount);
                curr_command_name = pieces[0];
                curr_command_reg_value = add_amount;
                curr_command_remaining_cycles = 2;
            }
        }

        // During cycle
        if (cycle_count - signal_strength_start_count) % signal_strength_period == 0 {
            println!("Time to calculate signal strength.");
            let signal_strength = cycle_count * x_reg_value;
            println!("signal_strength={}", signal_strength);
            sum_of_signal_strengths += signal_strength;
        }



        if curr_command_remaining_cycles > 0 {
            curr_command_remaining_cycles -= 1;
        }

        if curr_command_remaining_cycles == 0 {
            // Command has reached the end of it's running time
            if curr_command_name == "addx" {
                x_reg_value += curr_command_reg_value;
                println!("Update value of reg X to {} (end of cycle {})", x_reg_value, cycle_count);
            }
        }
        cycle_count += 1;
    }
    println!("Sum of signal strengths = {}", sum_of_signal_strengths);
}