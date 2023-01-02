use std::fs;
use std::collections::{HashSet, HashMap, BinaryHeap, VecDeque, BTreeSet};

use regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbours: HashSet<String>,
}

#[derive(Debug)]
struct State<'a> {
    curr_room: &'a str,
    opened: BTreeSet<String>,
    elapsed_time: u32,
    total_relieved_pressure: u32,
    // Rather than calculating this each time from the opened valves, it'll be quicker to keep track
    // of the cululmative sum of all opened valves
    pressure_per_min: u32,
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

    // Filter out rooms that have 0 flow rate
    let positive_flow_room_names: HashSet<_> = room_data.iter().filter(| (_, valve) | {
            valve.flow_rate > 0
        })
        .map(| (room_name, _) | {
            room_name
        }).collect();
    println!("positive_flow_room_names={:?}", positive_flow_room_names);

    part1(&room_data, &positive_flow_room_names);
    part2(&room_data, &positive_flow_room_names);
}
    
fn part1(room_data: &HashMap<String, Valve>, positive_flow_room_names: &HashSet<&String>) {

    // Need to find shortest paths between all rooms
    // assert!(find_min_cost("AA", "CC", &room_data) == 2);
    // assert!(find_min_cost("AA", "DD", &room_data) == 1);
    let mut states = VecDeque::new();

    // Initial state, start at "AA", with all valves closed, 0 elapsed time and pressure
    states.push_back(State {
        curr_room: "AA",
        opened: BTreeSet::new(),
        elapsed_time: 0,
        total_relieved_pressure: 0,
        pressure_per_min: 0,
    });

    // Keeps track of the max. pressure released from any state
    let mut max_pressure_released = 0u32;

    let mut count = 0;
    while let Some(curr_state) = states.pop_front() {
        if count == 1000 {
            println!("Processing state. curr_state={:?}", curr_state);
            println!("Queue length = {}", states.len());
            println!("Max released pressure = {}", max_pressure_released);
            count = 0;
        }
        count += 1;

        // Go from this room to all rooms which still have vavles closed and
        // flow rate > 0
        let unopened_pos_flow_room_names: Vec<_> = positive_flow_room_names.iter().filter(| &room_name | {
            !curr_state.opened.contains(*room_name)
        }).collect();
        // println!("unopened_pos_flow_room_names={:?}", unopened_pos_flow_room_names);

        // If the remaining rooms were all opened in the 
        // let total_remaining_flow_rate: u32 = unopened_pos_flow_room_names.iter().map(| &room_name | {
        //     room_data[*room_name].flow_rate
        // }).sum();
        // let pot_max = curr_state.total_relieved_pressure + total_remaining_flow_rate * (30 - curr_state.elapsed_time);
        let mut flow_rates: Vec<u32> = unopened_pos_flow_room_names.iter().map(| &room_name | {
            room_data[*room_name].flow_rate
        }).collect();
        flow_rates.sort(); // Highest flow rates will be last
        let mut relieved_pressure = curr_state.total_relieved_pressure;
        let mut pressure_per_min = curr_state.pressure_per_min;
        let mut time_spent = curr_state.elapsed_time;
        loop {
            time_spent += 2; // Assume it takes only 1min to move to room, and of course 1 min. to open
            if time_spent >= 30 {
                break;
            }
            relieved_pressure += 2*pressure_per_min;
            match flow_rates.pop() { // Get next highest value
                Some(i) => pressure_per_min += i,
                None => break,
            }
        }
        if relieved_pressure < max_pressure_released {
            // println!("Current state cannot possibly lead to something which beats the max, abondoning state.");
            continue;
        }

        // Add new states based on unopened valve rooms we could travel to.
        // 1 new state for every room we could visit
        for next_room_name in unopened_pos_flow_room_names {
            // We have not opened the valve in this room
            let mut opened_new_state = curr_state.opened.clone();
            let travel_time = find_min_cost(curr_state.curr_room, &next_room_name, &room_data);
            // Record that this valve in the next room is now open
            opened_new_state.insert(next_room_name.to_string());
            let new_total_relieved_pressure = curr_state.total_relieved_pressure + (travel_time + 1)*curr_state.pressure_per_min;
            // Increment the new pressure_per_min based on how much more opening this new rooms valve will do
            let new_pressure_per_min = curr_state.pressure_per_min + room_data[*next_room_name].flow_rate;

            let new_elapsed_time = curr_state.elapsed_time + travel_time + 1;

            if new_elapsed_time > 30 {
                // This new state exceeds the total run time, so it's not a valid state, skip
                // to next possible state
                // println!("New state would exceed max. time, not creating.");
                continue;
            }

            // Create new state
            let new_state = State {
                curr_room: &next_room_name,
                opened: opened_new_state,
                elapsed_time: curr_state.elapsed_time + travel_time + 1, // Add one for opening valve once we get there
                total_relieved_pressure: new_total_relieved_pressure,
                pressure_per_min: new_pressure_per_min,
            };
            // println!("Created new state, which will be pushed onto back of states vector. state={:?}", new_state);
            states.push_back(new_state);
        }

        // Now let this current state expire to the end of the 30mins, finding the total relieved pressure
        let remaining_time = 30 - curr_state.elapsed_time;
        let total_relieved_pressure = curr_state.total_relieved_pressure + remaining_time*curr_state.pressure_per_min;
        // println!("Let current state run to 30mins. total_released_pressure={}", total_relieved_pressure);
        if total_relieved_pressure > max_pressure_released {
            // We've found a new max!
            max_pressure_released = total_relieved_pressure;
        }

    }
    
    assert!(max_pressure_released == 2080, "Incorrect answer.");
    println!("part 1: max released pressure = {}", max_pressure_released);


}

fn part2(room_data: &HashMap<String, Valve>, positive_flow_room_names: &HashSet<&String>) {

    let run_time_mins = 26;
    let mut states = VecDeque::new();

    // Initial state, start at "AA", with all valves closed, 0 elapsed time and pressure
    states.push_back(State {
        curr_room: "AA",
        opened: BTreeSet::new(),
        elapsed_time: 0,
        total_relieved_pressure: 0,
        pressure_per_min: 0,
    });

    let mut max_pressure_relieved_by_open_valves: HashMap<BTreeSet<String>, u32> = HashMap::new();

    let mut count = 0;
    while let Some(curr_state) = states.pop_front() {
        if count == 1000 {
            println!("Processing state. curr_state={:?}", curr_state);
            println!("Queue length = {}", states.len());
            count = 0;
        }
        count += 1;

        // Go from this room to all rooms which still have vavles closed and
        // flow rate > 0
        let unopened_pos_flow_room_names: Vec<_> = positive_flow_room_names.iter().filter(| &room_name | {
            !curr_state.opened.contains(*room_name)
        }).collect();
        // println!("unopened_pos_flow_room_names={:?}", unopened_pos_flow_room_names);

        // Add new states based on unopened valve rooms we could travel to.
        // 1 new state for every room we could visit
        for next_room_name in unopened_pos_flow_room_names {
            // We have not opened the valve in this room
            let mut opened_new_state = curr_state.opened.clone();
            let travel_time = find_min_cost(curr_state.curr_room, &next_room_name, &room_data);
            // Record that this valve in the next room is now open
            opened_new_state.insert(next_room_name.to_string());
            let new_total_relieved_pressure = curr_state.total_relieved_pressure + (travel_time + 1)*curr_state.pressure_per_min;
            // Increment the new pressure_per_min based on how much more opening this new rooms valve will do
            let new_pressure_per_min = curr_state.pressure_per_min + room_data[*next_room_name].flow_rate;

            let new_elapsed_time = curr_state.elapsed_time + travel_time + 1;

            if new_elapsed_time > run_time_mins {
                // This new state exceeds the total run time, so it's not a valid state, skip
                // to next possible state
                // println!("New state would exceed max. time, not creating.");
                continue;
            }

            // Create new state
            let new_state = State {
                curr_room: &next_room_name,
                opened: opened_new_state,
                elapsed_time: curr_state.elapsed_time + travel_time + 1, // Add one for opening valve once we get there
                total_relieved_pressure: new_total_relieved_pressure,
                pressure_per_min: new_pressure_per_min,
            };
            // println!("Created new state, which will be pushed onto back of states vector. state={:?}", new_state);
            states.push_back(new_state);
        }

        // Now let this current state expire to the end of the run time, finding the total relieved pressure
        let remaining_time = run_time_mins - curr_state.elapsed_time;
        let total_relieved_pressure = curr_state.total_relieved_pressure + remaining_time*curr_state.pressure_per_min;

        // Save max pressure relieved by this open valve combo
        max_pressure_relieved_by_open_valves
            .entry(curr_state.opened.clone())
            .and_modify(| relieved_pressure | { *relieved_pressure = (*relieved_pressure).max(total_relieved_pressure) })
            .or_insert(total_relieved_pressure);
    }

    let max_pressure_released = max_pressure_relieved_by_open_valves
        .iter()
        .tuple_combinations()
        .filter(| (human, elephant) | human.0.is_disjoint(elephant.0))
        .map(| (human, elephant) | human.1 + elephant.1)
        .max()
        .unwrap();
    
    assert!(max_pressure_released == 2752, "Incorrect answer.");
    println!("part 2: max released pressure = {}", max_pressure_released);


}

fn check_debug(curr_valve_order: &Vec<String>, debug_valve_order: &Vec<&str>) -> bool {
    for i in 0..curr_valve_order.len() {
        if curr_valve_order[i] != debug_valve_order[i] {
            return false;
        }
    }
    return true;
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