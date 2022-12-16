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

    let mut pixels = String::from("");

    loop {
        if curr_command_remaining_cycles == 0 {
             // Time to grab another instruction
            let line;
            match lines.next() {
                None => {
                    break;
                },
                Some(i) => line = i,
            }
            if line == "noop" {
                curr_command_name = line;
                curr_command_remaining_cycles = 1;
            } else if line.starts_with("addx") {
                let pieces: Vec<&str> = line.split(" ").collect();
                let add_amount = pieces[1].parse::<i32>().unwrap();
                curr_command_name = pieces[0];
                curr_command_reg_value = add_amount;
                curr_command_remaining_cycles = 2;
            }
        }

        // During cycle
        if (cycle_count - signal_strength_start_count) % signal_strength_period == 0 {
            let signal_strength = cycle_count * x_reg_value;
            sum_of_signal_strengths += signal_strength;
        }

        // Do pixels stuff
        // Look at current sprite position
        let pixel_horiz_pos = (cycle_count - 1) % 40;
        if pixel_horiz_pos >=  x_reg_value - 1 && pixel_horiz_pos <= x_reg_value + 1 {
            pixels += "#";
        } else {
            pixels += ".";
        }
        if pixel_horiz_pos == 39 {
            pixels += "\n";
        }

        if curr_command_remaining_cycles > 0 {
            curr_command_remaining_cycles -= 1;
        }

        if curr_command_remaining_cycles == 0 {
            // Command has reached the end of it's running time
            if curr_command_name == "addx" {
                x_reg_value += curr_command_reg_value;
            }
        }
        cycle_count += 1;
    }
    println!("part 1: Sum of signal strengths = {}", sum_of_signal_strengths);
    println!("part 2: Pixels = \n{}", pixels);
}