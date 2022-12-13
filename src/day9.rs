use std::{fs, collections::HashSet};

pub fn run() {
    println!("day9");
    let contents = fs::read_to_string("src/day9.txt").unwrap();
    let part_1_tail_positions = run_with_knots(contents.as_str(), 2);
    println!("part 1: num. tail positions = {}", part_1_tail_positions);
    let part_2_tail_positions = run_with_knots(contents.as_str(), 10);
    println!("part 2: num. tail positions = {}", part_2_tail_positions);
}

pub fn run_with_knots(input: &str, num_knots: usize) -> usize {

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut knot_positions: Vec<(i32, i32)> = vec![(0, 0); num_knots];

    // Record initial tail position
    tail_positions.insert(*knot_positions.last().unwrap());

    for line in input.lines() {
        let pieces: Vec<&str> = line.split(" ").collect();
        let direction = pieces[0];
        let num_steps = pieces[1].parse::<u32>().unwrap();
        
        for _ in 0..num_steps {
            // Update first knot (head)
            let mut head_knot = knot_positions.get_mut(0).unwrap();
            if direction == "R" {
                head_knot.0 += 1;
            } else if direction == "L" {
                head_knot.0 -= 1;
            } else if direction == "U" {
                head_knot.1 += 1;
            } else if direction == "D" {
                head_knot.1 -= 1;
            }

            // Now check if other knots need updating
            for i in 1..knot_positions.len() {
                let prev_knot = knot_positions[i - 1];
                let mut curr_knot = knot_positions.get_mut(i).unwrap();
                let x_diff: i32 = prev_knot.0 - curr_knot.0;
                let y_diff: i32 = prev_knot.1 - curr_knot.1;
                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    curr_knot.0 += x_diff.signum();
                    curr_knot.1 += y_diff.signum();
                }
            }
            // Record tail position
            tail_positions.insert(*knot_positions.last().unwrap());
        }
    }
    return tail_positions.len();
}