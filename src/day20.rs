use std::fs;

use indoc::indoc;

pub fn run() {
    println!("day20");
    let use_example = false;

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

    // PART 1

    let sum_of_grove_coordinates_part1 = mix_and_find_sum_of_coords(&unmod_input, 1);

    println!("part 1: sum of grove coordinates = {}", sum_of_grove_coordinates_part1);
    if use_example {
        assert!(sum_of_grove_coordinates_part1 == 3);
    } else {
        assert!(sum_of_grove_coordinates_part1 == 10831);
    }

    // PART 2

    let decryption_key: i64 = 811589153;

    let unmod_input_with_key: Vec<_> = unmod_input
        .iter()
        .map(|&num| {
            num*decryption_key
        }).collect();

    let sum_of_grove_coordinates_part2 = mix_and_find_sum_of_coords(&unmod_input_with_key, 10);

    println!("part 2: sum of grove coordinates = {}", sum_of_grove_coordinates_part2);
    if use_example {
        assert!(sum_of_grove_coordinates_part2 == 1623178306);
    } else {
        assert!(sum_of_grove_coordinates_part2 == 6420481789383);
    }

}

fn mix_and_find_sum_of_coords(unmod_input: &Vec<i64>, num_times: i64) -> i64 {
    let mut mixed_list: Vec<_> = (0..unmod_input.len()).collect();

    for _ in 0..num_times {
        for (unmod_input_idx, unmod_input) in unmod_input.iter().enumerate() {
            // Find out where this unmod value is in the mixed list
            let mixed_list_idx = mixed_list.iter().position(|&mixed_list_val| {
                return mixed_list_val == unmod_input_idx; 
            }).unwrap();

            // Remove from mixed list
            mixed_list.remove(mixed_list_idx);

            // Insert back into list at moved location, wrapping as needed
            let new_mixed_list_idx = (mixed_list_idx as i64 + unmod_input).rem_euclid(mixed_list.len() as i64) as usize;
            mixed_list.insert(new_mixed_list_idx, unmod_input_idx);
        }
    }

    // Now we have to find the 0, there should be 1 of these in the original numbers
    let unmod_input_0_idx = unmod_input
        .iter()
        .position(|&number| {
            number == 0
        }).unwrap();

    // Find the position of the 0 in the mixed list
    let mixed_list_0_idx = mixed_list
        .iter()
        .position(|&number| {
            number == unmod_input_0_idx
        }).unwrap();

    let sum_of_grove_coordinates: i64 = [1000, 2000, 3000].iter().map(|num| {
        let unmod_input_idx = mixed_list[(mixed_list_0_idx + num) % mixed_list.len()];
        return unmod_input[unmod_input_idx];
    }).sum();

    return sum_of_grove_coordinates;
}