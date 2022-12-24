use std::fs;
use std::collections::{HashSet, HashMap};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow: u32,
    neighbours: String,
}

pub fn run() {
    println!("day16");
    let data: HashMap<String, Valve> = fs::read_to_string("src/day16.txt")
        .unwrap()
        .lines()
        .map(| line | {
            println!("test");
            let re = Regex::new("^Valve (.*?) has flow rate=(.*?); tunnel(s?) lead(s?) to valve(s?) (.*?)$").unwrap();
            let captures = re.captures(line).unwrap();
            let valve_name = captures.get(1).unwrap().as_str();
            println!("valve_name={}", valve_name);
            return ("hsh".to_string(), Valve{flow: 12, neighbours: "dd".to_string()})
        }).collect();

    println!("contents={:?}", data);

}