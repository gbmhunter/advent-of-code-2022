use std::fs;

pub fn run() {
    println!("day4");

    // Part 1
    let contents = fs::read_to_string("src/day4.txt").unwrap();
    let mut num_ranges_containing = 0;
    let mut num_ranges_overlapping = 0;
    for line in contents.lines() {        
        let ranges: Vec<&str> = line.split(",").collect();
        let range1 = parse_range(ranges[0]);
        let range2 = parse_range(ranges[1]);
        if range1.contains(&range2) || range2.contains(&range1) {
            num_ranges_containing += 1;
        }
        if range1.overlaps(&range2) {
            num_ranges_overlapping += 1;
        }
    }
    println!("part 1: num_ranges_containing={:?}", num_ranges_containing);
    println!("part 2: num_ranges_overlapping={:?}", num_ranges_overlapping);
}

#[derive(Debug)]
struct Range {
    low_limit: u32,
    high_limit: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.low_limit <= other.low_limit && self.high_limit >= other.high_limit
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.low_limit > other.high_limit || self.high_limit < other.low_limit)
    }
}

fn parse_range(input: &str) -> Range {
    let limits: Vec<&str> = input.split("-").collect();
    let low_limit = limits[0].parse::<u32>().unwrap();
    let high_limit = limits[1].parse::<u32>().unwrap();

    let range = Range {
        low_limit: low_limit,
        high_limit: high_limit,
    };
    range
}
