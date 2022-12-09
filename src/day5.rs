use std::fs;

pub fn run() {
    // Part 1
    let contents = fs::read_to_string("src/day5.txt").unwrap();

    let line_vec: Vec<&str> = contents.lines().collect();

    let mut empty_line_num = 0;

    // Initialize empty stacks
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..9 {
        // We know there only 9
        stacks.push(vec![]);
    }

    let mut found_end_of_chars = false;
    let mut idx = 0;
    while idx < line_vec.len() {
        let line = line_vec[idx];
        println!("\"{}\"", line);
        if !found_end_of_chars {
            let chars: Vec<char> = line.chars().collect();
            println!("{:?}", chars);

            let mut curr_pos = 1; // First char of any use
            let mut stack_idx = 0;
            while curr_pos < chars.len() {
                // Check if there is a char here or just empty space (becuase stack
                // is small enough that it hasn't started yet)
                if chars[curr_pos] == '1' {
                    println!("Found end of chars, found numbered row.");
                    found_end_of_chars = true;
                    // Need to reverse stacks as we created them in reverse
                    // order
                    for stack in &mut stacks {
                        // We know there only 9
                        stack.reverse();
                    }
                    idx += 1; // Increment to first instruction line
                    break;
                }
                if chars[curr_pos] != ' ' {
                    println!(
                        "Found useful char {} at position {}",
                        chars[curr_pos], curr_pos
                    );
                    stacks[stack_idx].push(chars[curr_pos])
                }
                curr_pos += 4; // Next useful char position
                stack_idx += 1;
            }
        } else {
            println!("Found instruction line.");
            println!("Stacks are currently: {:?}", stacks);
            let words: Vec<&str> = line.split(" ").collect();
            println!("{:?}", words);
            let num_crates_to_move: u32 = words[1].parse().unwrap();
            let from_stack_idx: usize = words[3].parse::<usize>().unwrap() - 1;
            let to_stack_idx: usize = words[5].parse::<usize>().unwrap() - 1;
            println!("moving {} crates from {} to {}", num_crates_to_move, from_stack_idx, to_stack_idx);
            for i in 0..num_crates_to_move {
                let crate_to_move = stacks[from_stack_idx].pop().unwrap();
                stacks[to_stack_idx].push(crate_to_move);
            }
        }
        

        idx += 1;
    }
    println!("finished stacking! stacks={:?}", stacks);
    // Create string of crates on top of stacks
    let crates_on_top: String = stacks.iter().map(| x | x.last().unwrap() ).collect();
    println!("crates_on_top={:?}", crates_on_top);


}
