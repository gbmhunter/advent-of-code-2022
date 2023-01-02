use std::{fs, collections::{HashSet, VecDeque}};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Coord(i32, i32, i32);

pub fn run() {
    println!("day18");
    let input = fs::read_to_string("src/day18.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut map: HashSet<Coord> = HashSet::new();
    for line in input.lines() {
        let coords: Vec<_> = line.split(",").collect();
        let x: i32 = coords[0].parse().unwrap();
        let y: i32 = coords[1].parse().unwrap();
        let z: i32 = coords[2].parse().unwrap();
        map.insert(Coord(x, y, z));
    }

    let mut num_sides_exposed = 0;
    for coord in &map {

        let coords_to_check = vec![
            Coord(coord.0 - 1, coord.1, coord.2),
            Coord(coord.0 + 1, coord.1, coord.2),
            Coord(coord.0, coord.1 - 1, coord.2),
            Coord(coord.0, coord.1 + 1, coord.2),
            Coord(coord.0, coord.1, coord.2 - 1),
            Coord(coord.0, coord.1, coord.2 + 1),
        ];
        // Check neighbouring spaces
        for coord_to_check in &coords_to_check {
            if !map.contains(coord_to_check) {
                num_sides_exposed += 1;
            }
        }
    }
    println!("part 1: num. sides exposed = {}", num_sides_exposed);
    assert!(num_sides_exposed == 3636);
}

fn part2(input: &str) {
    let mut map: HashSet<Coord> = HashSet::new();
    for line in input.lines() {
        let coords: Vec<_> = line.split(",").collect();
        let x: i32 = coords[0].parse().unwrap();
        let y: i32 = coords[1].parse().unwrap();
        let z: i32 = coords[2].parse().unwrap();
        map.insert(Coord(x, y, z));
    }
    let min_x = map.iter().map(|value| {
        value.0
    }).min().unwrap();
    let max_x = map.iter().map(|value| {
        value.0
    }).max().unwrap();
    let min_y = map.iter().map(|value| {
        value.1
    }).min().unwrap();
    let max_y = map.iter().map(|value| {
        value.1
    }).max().unwrap();
    let min_z = map.iter().map(|value| {
        value.2
    }).min().unwrap();
    let max_z = map.iter().map(|value| {
        value.2
    }).max().unwrap();
    // println!("min_x={}, max_x={}, min_y={}, max_y={}, min_z={}, max_z={}",
        // min_x, max_x, min_y, max_y, min_z, max_z);

    // Dynamic programming
    let starting_external_pixel = Coord(min_x - 1, min_y - 1, min_z - 1);
    // Queue of external pixels we need to check (dynamically added to as we walk around the exterior)
    let mut ext_pixel_queue: VecDeque<Coord> = VecDeque::new();
    // Need to keep track of external pixels we have already visited
    let mut ext_pixels_checked: HashSet<Coord> = HashSet::new();
    ext_pixel_queue.push_back(starting_external_pixel);

    let mut num_faces_on_surface = 0;


    while let Some(pixel_to_check) = ext_pixel_queue.pop_front() {
        // println!("Checking pixel {:?}", pixel_to_check);
        // Check it's six sides
        let coords_to_check = vec![
            Coord(pixel_to_check.0 - 1, pixel_to_check.1, pixel_to_check.2),
            Coord(pixel_to_check.0 + 1, pixel_to_check.1, pixel_to_check.2),
            Coord(pixel_to_check.0, pixel_to_check.1 - 1, pixel_to_check.2),
            Coord(pixel_to_check.0, pixel_to_check.1 + 1, pixel_to_check.2),
            Coord(pixel_to_check.0, pixel_to_check.1, pixel_to_check.2 - 1),
            Coord(pixel_to_check.0, pixel_to_check.1, pixel_to_check.2 + 1),
        ];

        for neighbour_pixels_to_check in &coords_to_check {
            // Bounds check
            if      neighbour_pixels_to_check.0 < min_x - 1 || neighbour_pixels_to_check.0 > max_x + 1 ||
                    neighbour_pixels_to_check.1 < min_y - 1 || neighbour_pixels_to_check.1 > max_y + 1 ||
                    neighbour_pixels_to_check.2 < min_z - 1 || neighbour_pixels_to_check.2 > max_z + 1 {
                // println!("Pixel {:?} is out of bounds.", neighbour_pixels_to_check);
                continue;
            }

            // Is in lava ball check
            if map.contains(neighbour_pixels_to_check) {
                // Pixel is in surface of lava ball.
                num_faces_on_surface += 1;
                continue;
            }

            // Already visited check
            if ext_pixels_checked.insert(neighbour_pixels_to_check.clone()) {
                // Was not already visited, add to queue
                ext_pixel_queue.push_back(neighbour_pixels_to_check.clone());
            }
        }


    
    }

    println!("part 2: num. sides of surface = {}", num_faces_on_surface);
    assert!(num_faces_on_surface == 2102);
}