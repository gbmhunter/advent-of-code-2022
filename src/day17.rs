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
    rock_shapes.push(
        vec![
            vec!['#', '#', '#', '#'],
        ]);
    rock_shapes.push(
        vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
        ]);
    rock_shapes.push(
        vec![
            vec!['.', '.', '#'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ]);
    rock_shapes.push(
        vec![
            vec!['#'],
            vec!['#'],
            vec!['#'],
            vec!['#'],
        ]);
    rock_shapes.push(
        vec![
            vec!['#', '#'],
            vec!['#', '#'],
        ]);
    println!("rock_shapes={:#?}", rock_shapes);

    // Map height is added to dynamically as needed
    // map[y][x]
    // y
    // |
    // O--x
    let mut map: Vec<Vec<char>> = Vec::new();



    let mut rock_index_to_insert = 0;
    let mut jet_index = 0;

    loop {
        // INSERT ROCK
        insert_new_rock(&mut map, &rock_shapes[rock_index_to_insert]);
        // Increment to next rock for next insertion
        rock_index_to_insert = (rock_index_to_insert + 1)%rock_shapes.len();
        
        // JET ROCK SIDE TO SIDE
        if jet_directions[jet_index..jet_index+1] == *"<" {
            // Go left
        } else {
            // Go right
        }
        jet_index = (jet_index + 1) % jet_directions.len();

        print_map(&map);
        break;
    }


}

fn insert_new_rock(map: &mut Vec<Vec<char>>, rock_to_insert: &Vec<Vec<char>>) {
    // Find highest rock row
    let mut highest_rock_row = 0;
    for row in map.iter() {
        if ! row.contains(&'#') {
            break;
        }
        highest_rock_row += 1;
    }
    println!("highest_rock_row={}", highest_rock_row);

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

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn move_rock(map: &mut Vec<Vec<char>>, direction: Direction) -> bool {
    // Make sure move is legal
    if direction == Direction::Left {
        let y_iter = map.iter();
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