use std::fs;

pub fn run() {
    println!("day17");
    let input = fs::read_to_string("src/day17.txt");

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
    let mut map: Vec<Vec<char>> = Vec::new();

    let map_width = 7;

    let starting_distance_left = 2;
    let starting_distance_bottom = 3;

    let rock_index_to_insert = 0;

    loop {
        // Insert rock
        map.push(vec!['.'; map_width]);
        print_map(&map);
        break;
    }


}

fn print_map(map: &Vec<Vec<char>>) {
    let mut output: String = String::new();
    for row in map {
        let mut row_output = String::new();
        for char in row {
            row_output += &char.to_string();
        }
        output += &row_output;
        output += "\n";
    }
    println!("map=\n{}", output);
}