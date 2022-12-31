use std::fs;

const map_width: usize = 7;

const starting_distance_left: usize = 2;
const starting_distance_bottom: usize = 3;

pub fn run() {
    println!("day17");
    let input = fs::read_to_string("src/day17.txt").unwrap();
    let jet_directions = input;

    let num_of_rocks_to_stop = 2022;

    let mut rock_shapes: Vec<Vec<Vec<char>>> = Vec::new();
    rock_shapes.push(vec![vec!['#', '#', '#', '#']]);
    rock_shapes.push(vec![
        vec!['.', '#', '.'],
        vec!['#', '#', '#'],
        vec!['.', '#', '.'],
    ]);
    rock_shapes.push(vec![
        vec!['#', '#', '#'],
        vec!['.', '.', '#'],
        vec!['.', '.', '#'],
    ]);
    rock_shapes.push(vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']]);
    rock_shapes.push(vec![vec!['#', '#'], vec!['#', '#']]);
    println!("rock_shapes={:#?}", rock_shapes);

    // Map height is added to dynamically as needed
    // map[y][x]
    // y
    // |
    // O--x
    let mut map: Vec<Vec<char>> = Vec::new();

    let mut rock_index_to_insert = 0;
    let mut jet_index = 0;

    let mut debug_count = 0;

    let mut need_new_rock = true;
    let mut num_fallen_rocks = 0;

    loop {
        if need_new_rock {
            // INSERT ROCK
            insert_new_rock(&mut map, &rock_shapes[rock_index_to_insert]);
            // Increment to next rock for next insertion
            rock_index_to_insert = (rock_index_to_insert + 1) % rock_shapes.len();
            // println!("Rock inserted. map=");
            // print_map(&map);
            need_new_rock = false;
        }

        // JET ROCK SIDE TO SIDE
        if jet_directions[jet_index..jet_index + 1] == *"<" {
            // Go left
            move_rock(&mut map, Direction::Left);
        } else {
            // Go right
            move_rock(&mut map, Direction::Right);
        }
        // println!(
        //     "Finished jetting with {}. map=",
        //     &jet_directions[jet_index..jet_index + 1]
        // );
        // print_map(&map);
        jet_index = (jet_index + 1) % jet_directions.len();

        // MOVE ROCK DOWN
        let did_move_down = move_rock(&mut map, Direction::Down);
        // println!("Finished moving rock down. map=");
        // print_map(&map);

        if !did_move_down {
            // println!("Rock could not move downward, solidying...");
            solidify_rock(&mut map);
            // println!("Finished solidfying rock. map=");
            // print_map(&map);
            println!("Solidfied rock num. {}.", num_fallen_rocks);
            num_fallen_rocks += 1;
            need_new_rock = true;
        }

        if num_fallen_rocks == 2022 {
            break;
        }
        debug_count += 1;
    }

    // Now count how tall the tower of rocks is
    let rock_tower_height = map.iter().filter(| &row | {
        row.contains(&'#')
    }).count();
    assert!(rock_tower_height == 3151);
    println!("rock tower height = {}", rock_tower_height);
}

fn insert_new_rock(map: &mut Vec<Vec<char>>, rock_to_insert: &Vec<Vec<char>>) {
    // Find highest rock row
    let mut highest_rock_row = 0;
    for row in map.iter() {
        if !row.contains(&'#') {
            break;
        }
        highest_rock_row += 1;
    }
    // println!("highest_rock_row={}", highest_rock_row);

    // Bottom left of rock goes 3 higher
    // Fill map with some more '.'
    for _ in 0..(starting_distance_bottom + rock_to_insert.len()) {
        map.push(vec!['.'; map_width]);
    }
    let map_insertion_height = highest_rock_row + starting_distance_bottom;

    // We now have enough '.' in map to accomodate new rock, lets now copy across the '#'
    for y in 0..rock_to_insert.len() {
        for x in 0..rock_to_insert[0].len() {
            let pixel_to_insert = rock_to_insert[y][x];
            if pixel_to_insert == '#' {
                // Insert '@' to represent a rock which has not yet found it's resting place,
                // i.e. rock in motion
                map[map_insertion_height + y][starting_distance_left + x] = '@';
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn move_rock(map: &mut Vec<Vec<char>>, direction: Direction) -> bool {
    // println!("move_rock() called. direction={:?}", direction);
    let mut delta_x: isize = 0;
    let mut delta_y: isize = 0;
    let mut x_iter: Vec<usize>;
    let mut y_iter: Vec<usize>;
    match direction {
        Direction::Left => {
            delta_x = -1;
            delta_y = 0;
            x_iter = (0..map[0].len()).collect();
            y_iter = (0..map.len()).collect();
        }
        Direction::Right => {
            delta_x = 1;
            delta_y = 0;
            x_iter = (0..map[0].len()).rev().collect();
            y_iter = (0..map.len()).collect();
        }
        Direction::Up => {
            delta_x = 0;
            delta_y = 1;
            x_iter = (0..map[0].len()).collect();
            y_iter = (0..map.len()).rev().collect();
        }
        Direction::Down => {
            delta_x = 0;
            delta_y = -1;
            x_iter = (0..map[0].len()).collect();
            y_iter = (0..map.len()).collect();
        }
    }

    // Make sure move is legal
    // println!("Making sure move is legal...");
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                // Found moving rock pixel
                let new_x = (x as isize + delta_x);
                let new_y = (y as isize + delta_y);
                // Don't need to check to y exceeding len() as bricks only ever move toward y=0
                if new_x < 0 || new_x >= map[0].len() as isize || new_y < 0 {
                    // Oh oh, hit the wall
                    // println!(
                    //     "Moving rock pixel at ({},{}) is going to collide into the wall at ({},{})",
                    //     x, y, new_x, new_y
                    // );
                    return false;
                }
                // Check pixel that it's going to move to is not rock
                if map[new_y as usize][new_x as usize] == '#' {
                    // Oh oh, found rock
                    // print!(
                    //     "Moving rock pixel at ({},{}) is going to collide into ({},{})",
                    //     x, y, new_x, new_y
                    // );
                    return false;
                }
            }
        }
    }
    // println!("Move is legal.");

    // If we get here, move is legal, it's move.
    // Have to be careful moving, iterator must be in the direction as to not
    // overwrite over moving pixels
    for y in y_iter.clone() {
        for x in x_iter.clone() {
            if map[y][x] == '@' {
                // println!(
                //     "Moving from ({},{}) to ({},{})",
                //     x,
                //     y,
                //     x as isize + delta_x,
                //     y as isize + delta_y
                // );
                map[y][x] = '.'; // Replace original location with nothing
                map[(y as isize + delta_y) as usize][(x as isize + delta_x) as usize] = '@';
            }
        }
    }

    // If we get here, rock moved successfully!
    return true;
}

fn solidify_rock(map: &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                map[y][x] = '#'; // Convert moving rock into solid rock
            }
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    let mut output: String = String::new();
    for row in map.iter().rev() {
        let mut row_output = String::new();
        for char in row {
            row_output += &char.to_string();
        }
        output += &row_output;
        output += "\n";
    }
    println!("map=\n{}", output);
}
