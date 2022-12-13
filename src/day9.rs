use std::{fs, num, collections::{HashMap, HashSet}};

pub fn run() {
    println!("day9");
    let contents = fs::read_to_string("src/day9.txt").unwrap();

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut tail_grid: Vec<Vec<bool>> = Vec::new();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    // Record initial tail position
    tail_positions.insert(tail_pos);

    for line in contents.lines() {
        let pieces: Vec<&str> = line.split(" ").collect();
        let direction = pieces[0];
        let num_steps = pieces[1].parse::<u32>().unwrap();
        println!("direction={}, num_steps={}", direction, num_steps);
        for i in 0..num_steps {
            if direction == "R" {
                head_pos.0 += 1;
            } else if direction == "L" {
                head_pos.0 -= 1;
            } else if direction == "U" {
                head_pos.1 += 1;
            } else if direction == "D" {
                head_pos.1 -= 1;
            }
            println!("Updated head position. Head position is now = {:?}", head_pos);

            // Now check if tail needs updating
            let x_diff: i32 = head_pos.0 - tail_pos.0;
            let y_diff: i32 = head_pos.1 - tail_pos.1;
            if x_diff.abs() > 1 {
                println!("Tail x needs moving!");
                tail_pos.0 += x_diff.signum();
                tail_pos.1 += y_diff.signum();
            }
            if y_diff.abs() > 1 {
                println!("Tail y needs moving!");
                tail_pos.0 += x_diff.signum();
                tail_pos.1 += y_diff.signum();
            }
            println!("Tail is now = {:?}", tail_pos);
            // Record tail position
            tail_positions.insert(tail_pos);
        }
    }
    println!("Num. of visited squares = {}", tail_positions.len());
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
        println!("direction={}, num_steps={}", direction, num_steps);
        
        for i in 0..num_steps {
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
            println!("Updated head knot position. Head knot pos is now = {:?}", head_knot);

            // Now check if other knots need updating
            for i in 1..knot_positions.len() {
                let prev_knot = knot_positions[i - 1];
                let mut curr_knot = knot_positions.get_mut(i).unwrap();
                let x_diff: i32 = prev_knot.0 - curr_knot.0;
                let y_diff: i32 = prev_knot.1 - curr_knot.1;
                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    println!("Knot {} needs moving!", i);
                    curr_knot.0 += x_diff.signum();
                    curr_knot.1 += y_diff.signum();
                }
                println!("Knot {} is now at = {:?}", i, curr_knot);
            }
            // Record tail position
            tail_positions.insert(*knot_positions.last().unwrap());
        }
    }
    println!("Num. of visited squares = {}", tail_positions.len());
    return tail_positions.len();
}