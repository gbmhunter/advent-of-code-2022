use std::fs;

use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn successors(&self) -> Vec<Pos> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
  }
}

static GOAL: Pos = Pos(4, 6);


pub fn run() {
    println!("day12");
    let contents = fs::read_to_string("src/day12.txt").unwrap();

    // Load characters into an 2D array
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_pos = Pos(-1, -1);
    let mut end_pos = Pos(-1, -1);
    let mut part_2_start_positions: Vec<Pos> = Vec::new();
    for (y, line) in contents.lines().enumerate() {
        let mut char_vec: Vec<char> = Vec::new();
        for (x, char) in line.chars().enumerate() {
            
            if char == 'S' {
                start_pos = Pos(x as i32, y as i32);
                part_2_start_positions.push(Pos(x as i32, y as i32));
                char_vec.push('a');
            } else if char == 'E' {
                end_pos = Pos(x as i32, y as i32);
                char_vec.push('z');
            } else if char == 'a' {
                part_2_start_positions.push(Pos(x as i32, y as i32));
                char_vec.push(char);
            } else {
                char_vec.push(char);
            }
        }
        map.push(char_vec);
    }

    // println!("Loaded map. map = {:?}", map);
    // println!("Start pos = {:?}", start_pos);
    // println!("End pos = {:?}", end_pos);
    
    // println!("Running pathfinding...");
    let result = bfs(&start_pos, |p| 
        calc_successors(p, &map), 
        |p| *p == end_pos);
    let min_num_steps = result.expect("no path found").len() - 1;
    println!("part 1: min. number of steps = {}", min_num_steps);

    // PART 2
    // Run for every starting point a
    let mut min_num_steps_vec: Vec<usize> = Vec::new();
    for part_2_start_pos in &part_2_start_positions {
        // println!("part_2_start_pos={:?}", part_2_start_pos);
        let result = bfs(part_2_start_pos, |p| 
            calc_successors(p, &map), 
            |p| *p == end_pos);
        match result {
            None => (),
            Some(t) => min_num_steps_vec.push(t.len() - 1),
        }
    }
    println!("part 2: min. number of steps from any starting position = {:?}", min_num_steps_vec.iter().min().unwrap());
}

fn calc_successors(curr_pos: &Pos, map: &Vec<Vec<char>>) -> Vec<Pos> {
    // println!("calc_successors() called.");
    // println!("curr_pos={:?}", curr_pos);
    let mut successors: Vec<Pos> = Vec::new();

    let elevation_at_curr_pos = map[curr_pos.1 as usize][curr_pos.0 as usize] as i32;
    // println!("elevation_at_curr_pos={}", elevation_at_curr_pos);
    // LEFT
    if curr_pos.0 >= 1 && map[curr_pos.1 as usize][(curr_pos.0 - 1) as usize] as i32 <= elevation_at_curr_pos + 1 {
        // println!("Can go LEFT");
        successors.push(Pos(curr_pos.0 - 1, curr_pos.1));
    }
    // RIGHT
    if curr_pos.0 <= (map[0].len() - 2) as i32 && map[curr_pos.1 as usize][(curr_pos.0 + 1) as usize] as i32 <= elevation_at_curr_pos + 1 {
        // println!("Can go RIGHT");
        successors.push(Pos(curr_pos.0 + 1, curr_pos.1));
    }
    // UP
    if curr_pos.1 >= 1 && map[(curr_pos.1 - 1) as usize][(curr_pos.0) as usize] as i32 <= elevation_at_curr_pos + 1 {
        // println!("Can go UP");
        successors.push(Pos(curr_pos.0, curr_pos.1 - 1));
    }
    // DOWN
    if curr_pos.1 <= (map.len() - 2) as i32 && map[(curr_pos.1 + 1) as usize][(curr_pos.0) as usize] as i32 <= elevation_at_curr_pos + 1 {
        // println!("Can go DOWN");
        successors.push(Pos(curr_pos.0, curr_pos.1 + 1));
    }
    return successors;
}