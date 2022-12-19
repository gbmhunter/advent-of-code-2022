use std::fs;

#[derive(Debug)]
struct Coord(usize, usize);

#[derive(Clone, PartialEq)]
enum PixelTypes {
    Nothing,
    Rock,
    Sand,
    SourceOfSand,
}

const SAND_POINT: Coord = Coord(500, 0);

pub fn run() {
    println!("day14");
    let contents = fs::read_to_string("src/day14.txt").unwrap();

    

    // Instantiate 2D map to hold rock and sand locations. We could scale (500, 0) to be (0, 0)
    // to save on a bit of memory but not really worth it
    let mut map: Vec<Vec<PixelTypes>> = vec![vec![PixelTypes::Nothing; 1000]; 1000];

    map[500][0] = PixelTypes::SourceOfSand;

    for line in contents.lines() {
        println!("{}", line);
        let pieces = line.split(" -> ").collect::<Vec<_>>();
        let rock_coords = pieces.iter().
            map(| piece | {
                let numbers = piece.split(",").collect::<Vec<_>>();
                return Coord(numbers[0].parse::<usize>().unwrap(), numbers[1].parse::<usize>().unwrap())
            }).collect::<Vec<_>>();
        println!("{:?}", rock_coords);
        for i in 0..rock_coords.len() - 1 {
            draw_line(&mut map, &rock_coords[i], &rock_coords[i+1]);
        }
    }
    print_map(&map);
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

fn produce_sand(map: &mut Vec<Vec<PixelTypes>>) {
    let mut sand_location = SAND_POINT;
    loop {
        if map[sand_location.0][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0, sand_location.1 + 1);
        } else if map[sand_location.0 - 1][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0 - 1, sand_location.1 + 1);
        } else if map[sand_location.0 + 1][sand_location.1 + 1] == PixelTypes::Nothing {
            sand_location = Coord(sand_location.0 + 1, sand_location.1 + 1);
        } else {
            println!("Sand has come to a rest!");
            map[sand_location.0][sand_location.1] = PixelTypes::Sand;
            return;
        }
    }
}

fn print_map(map: &Vec<Vec<PixelTypes>>) {
    let mut output = String::from("");
    for y in 0..10 {
        for x in 490..510 {
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