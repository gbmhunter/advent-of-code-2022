use std::fs;
use std::collections::{HashSet, HashMap, BinaryHeap, VecDeque};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbours: HashSet<String>,
}


struct State<'a> {
    curr_room: &'a str,
    opened: HashSet<String>,
    elapsed_time: u32,
    total_relieved_pressure: u32,
}

pub fn run() {
    println!("day16");
    let room_data: HashMap<String, Valve> = fs::read_to_string("src/day16.txt")
        .unwrap()
        .lines()
        .map(| line | {
            let re = Regex::new("^Valve (.*?) has flow rate=(.*?); tunnels? leads? to valves? (.*?)$").unwrap();
            let captures = re.captures(line).unwrap();
            let valve_name = captures.get(1).unwrap().as_str();
            let flow_rate = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let neighbours: HashSet<String> = captures.get(3).unwrap().as_str().split(",").map(|string| {
                return string.trim().to_string();
            }).collect();
            return (valve_name.to_string(), Valve{flow_rate, neighbours})
        }).collect();

    println!("data={:?}", room_data);

    // Need to find shortest paths between all rooms
    // assert!(find_min_cost("AA", "CC", &room_data) == 2);
    // assert!(find_min_cost("AA", "DD", &room_data) == 1);
    let mut states = VecDeque::new();

    // Initial state, start at "AA", with all valves closed, 0 elapsed time and pressure
    states.push_back(State {
        curr_room: "AA",
        opened: HashSet::new(),
        elapsed_time: 0,
        total_relieved_pressure: 0,
    });
    


}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32,
    room_name: &'a str,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Note how these are switched around, this gives us a min priority queue
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_cost(from: &str, to: &str, room_data: &HashMap<String, Valve>) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Node {
        cost: 0,
        room_name: from,
    });

    visited.insert(from);

    while let Some(Node { cost, room_name }) = queue.pop() {
        if room_name == to {
            // We're there!
            return cost;
        }
        for neighbour in &room_data[room_name].neighbours {
            if visited.insert(neighbour.as_str()) {
                queue.push(Node {
                    cost: cost + 1,
                    room_name: neighbour.as_str(),
                });
            }
        }
    }
    panic!("Should not get here");
}