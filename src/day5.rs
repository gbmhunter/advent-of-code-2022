use std::fs;

pub fn run() {
    // Part 1
    let contents = fs::read_to_string("src/day5.txt").unwrap();

    let line_vec: Vec<&str> = contents.lines().collect();

    // Initialize empty stacks
    let mut stacks_p1: Vec<Vec<char>> = vec![];
    let mut stacks_p2: Vec<Vec<char>> = vec![];
    for _ in 0..9 {
        // We know there only 9
        stacks_p1.push(vec![]);
    }

    let mut found_end_of_chars = false;
    let mut idx = 0;
    while idx < line_vec.len() {
        let line = line_vec[idx];
        if !found_end_of_chars {
            let chars: Vec<char> = line.chars().collect();
            let mut curr_pos = 1; // First char of any use
            let mut stack_idx = 0;
            while curr_pos < chars.len() {
                // Check if there is a char here or just empty space (becuase stack
                // is small enough that it hasn't started yet)
                if chars[curr_pos] == '1' {
                    found_end_of_chars = true;
                    // Need to reverse stacks as we created them in reverse
                    // order
                    for stack in &mut stacks_p1 {
                        // We know there only 9
                        stack.reverse();
                    }
                    // Copy this stack, as this is where part 2 begins to diverge
                    stacks_p2 = stacks_p1.clone();
                    idx += 1; // Increment to first instruction line
                    break;
                }
                if chars[curr_pos] != ' ' {
                    stacks_p1[stack_idx].push(chars[curr_pos])
                }
                curr_pos += 4; // Next useful char position
                stack_idx += 1;
            }
        } else {
            let words: Vec<&str> = line.split(" ").collect();
            let num_crates_to_move: u32 = words[1].parse().unwrap();
            let from_stack_idx: usize = words[3].parse::<usize>().unwrap() - 1;
            let to_stack_idx: usize = words[5].parse::<usize>().unwrap() - 1;
            let mut p2_temp_stack: Vec<char> = vec![];
            for _ in 0..num_crates_to_move {
                let mut crate_to_move = stacks_p1[from_stack_idx].pop().unwrap();
                stacks_p1[to_stack_idx].push(crate_to_move);

                crate_to_move = stacks_p2[from_stack_idx].pop().unwrap();
                p2_temp_stack.push(crate_to_move);
            }
            p2_temp_stack.reverse();
            stacks_p2[to_stack_idx].append(&mut p2_temp_stack);
        }
        

        idx += 1;
    }
    // Create string of crates on top of stacks
    let mut crates_on_top: String = stacks_p1.iter().map(| x | x.last().unwrap() ).collect();
    println!("part 1: crates_on_top={:?}", crates_on_top);

    crates_on_top = stacks_p2.iter().map(| x | x.last().unwrap() ).collect();
    println!("part 2: crates_on_top={:?}", crates_on_top);
}
