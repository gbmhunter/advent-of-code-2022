use std::{fs, num, collections::{HashMap, HashSet}};

pub fn run() {
    println!("day9");
    let contents = fs::read_to_string("src/day9.txt").unwrap();

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

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

            // Record tail position
            tail_positions.insert(tail_pos);

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
        }
    }
    printtail_positions.len()
}