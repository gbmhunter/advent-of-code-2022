use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    // [ore, clay, obsidian, geode]
    costs: [[i32; 4]; 4],
}

#[derive(Debug)]
struct State {
    elapsed_time_mins: i32,
    // [ore_robots, clay_robots, obsidian_robots, geode_robots]
    num_robots: [i32; 4],
    // [ore, clay, obsidian, geode]
    num_resources: [i32; 4],
}

const TOTAL_NUM_MINS: i32 = 24;

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
    let mut max_num_geodes = 0;

    while let Some(line) = lines.next() {
        // println!("{}", lines.next().unwrap());
        let ore_robot_cost: i32 = ore_robot_regex.captures(
                line).
                unwrap().get(1).unwrap().as_str().parse().unwrap();
        let clay_robot_cost: i32 = clay_robot_regex.captures(
                line).
                unwrap().get(1).unwrap().as_str().parse().unwrap();
        let captures = obsidian_robot_regex.captures(line).unwrap();
        let obsidian_robot_cost_ore: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let obsidian_robot_cost_clay: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let captures = geode_robot_regex.captures(line).unwrap();
        let geode_robot_cost_ore: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_obsidian: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

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
        elapsed_time_mins: 0,
        num_resources: [0, 0, 0, 0],
        num_robots: [1, 0, 0, 0],
    };

    let mut queue: Vec<State> = Vec::new();
    queue.push(initial_state);

    let blueprint = &blueprints[0];

    let mut debug_count = 0;

    while let Some(curr_state) = queue.pop() {
        if debug_count % 100000 == 0 {
            println!("processing state num. {}, queue len = {}, max num. geodes = {}", debug_count, queue.len(), max_num_geodes);
        }
        debug_count += 1;
        // println!("Processing new state. state = {:?}", curr_state);

        // What can we build?
        // Find the limiting resource (time wise) needed to make each robot (assuming we don't make
        // any other robot in the mean time)
        // 0mins = we can enough resources on hand to make it now
        // >0 mins = make it in the future once more resources are gathered
        // -1 = can't make it ever with current robots (i.e. we don't have even a single robot collecting
        // that resource)
        // First robot is ore robot, then clay robot, e.t.c
        for (robot_idx, robot_costs) in blueprint.costs.iter().enumerate() {

            // For the resource this robot generates, find the max. amount required
            // for any 1 minute to build any robot. If we already have enough of these
            // robots to generate this max. amount, do not create any more
            // (excl. geode robots, we always want these!)
            let max_req_num_of_these_robots = blueprint.costs.iter().map(|resource| {
                resource[robot_idx]
            }).max().unwrap();

            if robot_idx != 3 && curr_state.num_robots[robot_idx] >= max_req_num_of_these_robots {
                // println!("Already have enough of these robots, not making any more! idx = {}, num robots = {}, max req. = {}",
                //         robot_idx,
                //         curr_state.num_robots[robot_idx],
                //         max_req_num_of_these_robots);
                continue;
            }


            let mut num_days_for_resource: [i32; 4] = [0; 4];
            // For each resource required for robot
            for i in 0..robot_costs.len() {
                let remaining_to_collect = robot_costs[i] as i32 - curr_state.num_resources[i] as i32;
                if robot_costs[i] == 0 || remaining_to_collect < 0 {
                    num_days_for_resource[i] = 0;
                } else if curr_state.num_robots[i] == 0 {
                    // Oh oh, we don't have any of the robots required to gather this resource,
                    // so we are never going to be able to make it
                    num_days_for_resource[i] = i32::MAX;
                } else {
                    // Perform ceiling division
                    num_days_for_resource[i] = 
                        (remaining_to_collect + curr_state.num_robots[i] as i32 - 1) / curr_state.num_robots[i] as i32;
                }
            }
            // println!("Num. days to make robot {} is {:?}", robot_idx, num_days_for_resource);
            let num_days_to_get_all_resources = *num_days_for_resource.iter().max().unwrap();
            // println!("max days = {}", num_days_to_get_all_resources);

            if num_days_to_get_all_resources == i32::MAX {
                // Don't have the right robots to make the resources required for this robot,
                // so give up trying to build one
                // println!("Don't have the right robots to make {}.", robot_idx);
                continue;
            }

            // Make sure we can build one and it be useful before time runs out
            // + 1 to include time to build robot
            let time_to_advance_to = curr_state.elapsed_time_mins + num_days_to_get_all_resources + 1;

            if time_to_advance_to > TOTAL_NUM_MINS {
                // println!("Making this robot would exceed max. runtime, so not building.");
                continue;
            }

            let mut new_num_resources = [0; 4];
            for (i, resource) in curr_state.num_resources.iter().enumerate() {
                new_num_resources[i] = 
                        resource
                        - blueprint.costs[robot_idx][i] // Subtract of cost to build this new robot
                        + curr_state.num_robots[i] * (num_days_to_get_all_resources + 1); // Add resource collected by existing robots
            }

            let mut new_num_robots = curr_state.num_robots.clone();
            new_num_robots[robot_idx] += 1; // Add one for the robot we are just now making

            // Make robot, advance time, create new state
            let new_state = State {
                elapsed_time_mins: time_to_advance_to,
                num_resources: new_num_resources,
                num_robots: new_num_robots,
            };
            // println!("Made new game state! state = {:?}", new_state);
            queue.push(new_state);
        }

        // Run this state to completion to see how many geodes we get
        let num_mins_remaining = TOTAL_NUM_MINS - curr_state.elapsed_time_mins;
        assert!(num_mins_remaining >= 0);
        let num_of_geodes = curr_state.num_resources[3] + curr_state.num_robots[3]*num_mins_remaining;
        // println!("Num. geodes = {}", num_of_geodes);
        // Update max. if relevant
        max_num_geodes = max_num_geodes.max(num_of_geodes);

    }
    println!("part 1: max. num geodes = {}", max_num_geodes);

    
}