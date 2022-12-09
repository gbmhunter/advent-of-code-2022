// To run, type this in the Console: cargo run --example day1

use std::fs;

pub fn run() {
    println!("day1");

    // Part 1
    let contents = fs::read_to_string("src/day1.txt").unwrap();
    let lines = contents.lines();
    let mut calorie_sums: Vec<u32> = Vec::new();
    calorie_sums.push(0); // First elf doesn't get a new line
    for line in lines {
        if line == "" {
            calorie_sums.push(0); // Must be new elf, create new entry
        } else {
            let num = line.parse::<u32>().unwrap();
            let len = calorie_sums.len();
            calorie_sums[len - 1] += num;
        }
    }
    println!(
        "part 1: max_total_calories={:?}",
        calorie_sums.iter().max().unwrap()
    );

    // Part 2
    calorie_sums.sort();
    calorie_sums.reverse(); // Largest calories first
    let mut total_of_largest_3 = 0;
    for calorie_sum in calorie_sums.iter().take(3) {
        total_of_largest_3 += calorie_sum;
    }
    println!("part 2: total_of_largest_3={:?}", total_of_largest_3);
}
