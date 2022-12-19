use std::fs;

#[derive(Debug, PartialEq)]
struct Coord(usize, usize);

#[derive(Clone, PartialEq)]
enum PixelTypes {
    Nothing,
    Rock,
    Sand,
    SourceOfSand,
}

const SAND_ENTRY_POINT: Coord = Coord(500, 0);

pub fn run() {
    println!("day14");
    let contents = fs::read_to_string("src/day14.txt").unwrap();

    // Instantiate 2D map to hold rock and sand locations. We could scale (500, 0) to be (0, 0)
    // to save on a bit of memory but not really worth it
    let mut map: Vec<Vec<PixelTypes>> = vec![vec![PixelTypes::Nothing; 1000]; 1000];

    map[500][0] = PixelTypes::SourceOfSand;

    for line in contents.lines() {
        let pieces = line.split(" -> ").collect::<Vec<_>>();
        let rock_coords = pieces.iter().
            map(| piece | {
                let numbers = piece.split(",").collect::<Vec<_>>();
                return Coord(numbers[0].parse::<usize>().unwrap(), numbers[1].parse::<usize>().unwrap())
            }).collect::<Vec<_>>();
        for i in 0..rock_coords.len() - 1 {
            draw_line(&mut map, &rock_coords[i], &rock_coords[i+1]);
        }
    }

    // Find maximum y value for any rock
    let mut max_y = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == PixelTypes::Rock && y > max_y {
                max_y = y;
            }
        }
    }

    // PART 1

    let mut map_part_1 = map.clone();

    // Now insert and move sand
    let mut sand_stopped = true;
    let mut num_sand_particles = 0;
    while sand_stopped {
        sand_stopped = move_sand_part1(&mut map_part_1, max_y);
        num_sand_particles += 1;
    }
    num_sand_particles -= 1; // Subtract 1 off since we detect the first that falls forever
    assert!(num_sand_particles == 1068, "Incorrect answer.");
    println!("part 1: num. sand units = {}", num_sand_particles);

    // PART 2

    let mut map_part2 = map.clone();
    // Now insert and move sand
    let mut sand_stopped = true;
    let mut num_sand_particles = 0;
    let floor_y = max_y + 2;
    while sand_stopped {
        sand_stopped = move_sand_part2(&mut map_part2, floor_y);
        num_sand_particles += 1;
    }
    assert!(num_sand_particles == 27936, "Incorrect answer.");
    println!("part 2: num. sand units = {}", num_sand_particles);

}

fn draw_line(map: &mut Vec<Vec<PixelTypes>>, from: &Coord, to: &Coord) {
    let x_range;
    if from.0 < to.0 {
        x_range = from.0..to.0 + 1;
    } else {
        x_range = to.0..from.0 + 1;
    }
    let y_range;
    if from.1 < to.1 {
        y_range = from.1..to.1 + 1;
    } else {
        y_range = to.1..from.1 + 1;
    }
    for x in x_range {
        for y in y_range.clone() {
            map[x as usize][y as usize] = PixelTypes::Rock;
        }
    }
}

fn move_sand_part1(map: &mut Vec<Vec<PixelTypes>>, max_y: usize) -> bool {
    let mut sand_location = SAND_ENTRY_POINT;
    loop {
        if sand_location.1 + 1 > max_y {
            // Sand has exceeded max y bounds
            return false;
        }
        if map[sand_location.0][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0, sand_location.1 + 1);
        } else if map[sand_location.0 - 1][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0 - 1, sand_location.1 + 1);
        } else if map[sand_location.0 + 1][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0 + 1, sand_location.1 + 1);
        } else {
            map[sand_location.0][sand_location.1] = PixelTypes::Sand;
            return true;
        }
    }
}

fn move_sand_part2(map: &mut Vec<Vec<PixelTypes>>, floor_y: usize) -> bool {
    // Returns false when the sand entrance is blocked
    let mut sand_location = SAND_ENTRY_POINT;
    loop {
        if sand_location.1 + 1 == floor_y {
            // Sand is on top of floor
            map[sand_location.0][sand_location.1] = PixelTypes::Sand;
            return true;
        } else if map[sand_location.0][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0, sand_location.1 + 1);
        } else if map[sand_location.0 - 1][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0 - 1, sand_location.1 + 1);
        } else if map[sand_location.0 + 1][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0 + 1, sand_location.1 + 1);
        } else {
            map[sand_location.0][sand_location.1] = PixelTypes::Sand;
            if sand_location == SAND_ENTRY_POINT {
                // Sand can't move off sand entrance, so entrance is now blocked
                return false;
            } else {
                return true;
            }
        }
    }
}

fn print_map(map: &Vec<Vec<PixelTypes>>) {
    let mut output = String::from("");
    for y in 0..15 {
        for x in 485..515 {
            match map[x][y] {
                PixelTypes::Nothing => output += ".",
                PixelTypes::Rock => output += "#",
                PixelTypes::Sand => output += "*",
                PixelTypes::SourceOfSand => output += "+",
            }
        }
        output += "\n";
    }
    print!("{}", output);
}