use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    // [ore, clay, obsidian, geode]
    costs: [[u32; 4]; 4],
}

#[derive(Debug)]
struct State {
    elapsed_time: u32,
    // [ore_robots, clay_robots, obsidian_robots, geode_robots]
    num_robots: [u32; 4],
    // [ore, clay, obsidian, geode]
    num_resources: [u32; 4],
}

pub fn run() {
    println!("day19");

    let use_example = true;

    let input: String;
    if use_example {
        input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_string();
    } else {
        input = fs::read_to_string("src/day19.txt").unwrap();
    }

    let mut lines = input.lines();

    let ore_robot_regex = Regex::new("Each ore robot costs ([0-9]+) ore.").unwrap();
    let clay_robot_regex = Regex::new("Each clay robot costs ([0-9]+) ore.").unwrap();
    let obsidian_robot_regex = Regex::new("Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay.").unwrap();
    let geode_robot_regex = Regex::new("Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    while let Some(line) = lines.next() {
        // println!("{}", lines.next().unwrap());
        let ore_robot_cost: u32 = ore_robot_regex.captures(
                line).
                unwrap().get(1).unwrap().as_str().parse().unwrap();
        let clay_robot_cost: u32 = clay_robot_regex.captures(
                line).
                unwrap().get(1).unwrap().as_str().parse().unwrap();
        let captures = obsidian_robot_regex.captures(line).unwrap();
        let obsidian_robot_cost_ore: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let obsidian_robot_cost_clay: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let captures = geode_robot_regex.captures(line).unwrap();
        let geode_robot_cost_ore: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_obsidian: u32 = captures.get(2).unwrap().as_str().parse().unwrap();

        blueprints.push(Blueprint {
            costs: [
            [ore_robot_cost, 0, 0, 0],
            [clay_robot_cost, 0, 0, 0],
            [obsidian_robot_cost_ore, obsidian_robot_cost_clay, 0, 0],
            [geode_robot_cost_ore, 0, geode_robot_cost_obsidian, 0],
        ]});
    }
    println!("blueprints = {:?}", blueprints);

    let initial_state = State {
        elapsed_time: 0,
        num_resources: [0, 0, 0, 0],
        num_robots: [1, 0, 0, 0],
    };

    let mut queue: Vec<State> = Vec::new();
    queue.push(initial_state);

    let blueprint = &blueprints[0];

    while let Some(curr_state) = queue.pop() {
        println!("Processing new state. state = {:?}", curr_state);

        // What can we build?
        // First robot is ore robot, then clay robot, e.t.c
        for robot_costs in blueprint.costs {
            let mut can_make_robot = true;
            for i in 0..robot_costs.len() {
                if curr_state.num_resources[i] < robot_costs[i] {
                    can_make_robot = false;
                }
            }
            println!("Can make robot? {}", can_make_robot);
        }

        // Collect resources from robots

    }

    
}