use std::fs;
use std::ops::{Add, Sub};

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn manhattan(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

#[derive(Debug)]
struct Sensor {
    location: Coord,
    closest_beacon: Coord,
    manhatten: i32,
}

pub fn run() {
    println!("day15");
    let contents = fs::read_to_string("src/day15.txt").unwrap();

    let row_to_scan_y = 2000000;

    let mut sensors: Vec<Sensor> = Vec::new();
    for line in contents.lines() {
        // println!("Processing {}", line);
        let re = Regex::new(r"Sensor at x=(.*?), y=(.*?): closest beacon is at x=(.*?), y=(.*)").unwrap();
        let caps = re.captures(line).unwrap();

        let sensor_x = caps.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
        let sensor_y = caps.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap();

        let closest_beacon_x = caps.get(3).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
        let closest_beacon_y = caps.get(4).map_or("", |m| m.as_str()).parse::<i32>().unwrap();

        // println!("sensor_x={}, sensor_y={}, closest_beacon_x={}, closest_beacon_y={}", sensor_x, sensor_y, closest_beacon_x, closest_beacon_y);
        let location = Coord{x: sensor_x, y: sensor_y};
        let closest_beacon = Coord{x: closest_beacon_x, y: closest_beacon_y};
        let manhatten = (closest_beacon - location).manhattan();
        let sensor = Sensor { 
            location,
            closest_beacon,
            manhatten,
        };
        sensors.push(sensor);
    }
    // println!("sensors={:?}", sensors);
    // Work out what range of x values we need to scan
    let min_x = sensors.iter().map(|sensor| {
        let min_x = sensor.location.x - sensor.manhatten;
        return min_x;
    }).min().unwrap();
    println!("min_x={}", min_x);
    let max_x = sensors.iter().map(|sensor| {
        let max_x = sensor.location.x + sensor.manhatten;
        return max_x;
    }).max().unwrap();
    println!("max_x={}", max_x);

    let mut num_pos_beacon_cannot_be = 0;
    for x in min_x..max_x {
        let current_coord = Coord{x: x, y: row_to_scan_y};
        // println!("Testing point {:?}", current_coord);
        let mut in_exclusion_zone = false;
        let mut known_sensor_or_beacon_is_here = false;
        // At each point, test if we are in the exclusion zone of any sensor
        for sensor in &sensors {
            // println!("Looking as sensor {:?}", sensor);
            if current_coord == sensor.location || current_coord == sensor.closest_beacon {
                // Bail, since there is a known sensor or beacon at this point there can't
                // possibly be another beacon, not counted
                // println!("Known sensor or beacon is here!.");
                known_sensor_or_beacon_is_here = true;
                break;
            }
            if (current_coord - sensor.location).manhattan() <= sensor.manhatten {
                // We are in exclusion zone
                // println!("{:?} is in exclusion zone.", current_coord);
                in_exclusion_zone = true;
            }
        }
        if in_exclusion_zone && !known_sensor_or_beacon_is_here {
            // println!("{:?} is in exclusion zone and no known sensor or beacon is here.", current_coord);
            num_pos_beacon_cannot_be += 1;
        }
    }
    assert!(num_pos_beacon_cannot_be == 5100463, "Incorrect answer.");
    println!("part 1: num. of positions beacon cannot be = {}", num_pos_beacon_cannot_be);
}