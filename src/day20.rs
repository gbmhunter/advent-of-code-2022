use std::fs;

use indoc::indoc;

pub fn run() {
    println!("day20");
    let use_example = true;

    let input: String;
    if use_example {
        input = indoc! {"
            1
            2
            -3
            3
            -2
            0
            4
        "}.to_string();
    } else {
        input = fs::read_to_string("src/day20.txt").unwrap();
    }

    let unmod_input: Vec<_> = input.lines().map(|line| {
        return line.parse::<i64>().unwrap()
    }).collect();

    let mut mixed_list: Vec<_> = (0..unmod_input.len()).collect();

    println!("{:?}", unmod_input);

    for (unmod_input_idx, unmod_input) in unmod_input.iter().enumerate() {
        // Find out where this unmod value is in the mixed list
        let mixed_list_idx = mixed_list.iter().position(|&mixed_list_val| {
            return mixed_list_val == unmod_input_idx; 
        }).unwrap();

        // Remove from mixed list
        mixed_list.remove(mixed_list_idx);

        // Insert
        let new_mixed_list_idx = (mixed_list_idx as i64 + unmod_input).rem_euclid(mixed_list.len() as i64) as usize;
        mixed_list.insert(new_mixed_list_idx, unmod_input_idx);
    }


}