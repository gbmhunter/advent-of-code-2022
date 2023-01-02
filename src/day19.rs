use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost_ore: u32,
    obsidian_robot_cost_clay: u32,
    geode_robot_cost_ore: u32,
    geode_robot_cost_clay: u32,
}

pub fn run() {
    println!("day19");
    let input = fs::read_to_string("src/day19.txt").unwrap();

    let mut lines = input.lines();

    let ore_robot_regex = Regex::new("Each ore robot costs ([0-9]+) ore.").unwrap();
    let clay_robot_regex = Regex::new("Each clay robot costs ([0-9]+) ore.").unwrap();
    let obsidian_robot_regex = Regex::new("Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay.").unwrap();
    let geode_robot_regex = Regex::new("Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    loop {
        // First line we don't care about
        if lines.next().is_none() {
            break;
        }
        // println!("{}", lines.next().unwrap());
        let ore_robot_cost: u32 = ore_robot_regex.captures(
                lines.next().unwrap()).
                unwrap().get(1).unwrap().as_str().parse().unwrap();
        let clay_robot_cost: u32 = clay_robot_regex.captures(
                lines.next().unwrap()).
                unwrap().get(1).unwrap().as_str().parse().unwrap();
        let captures = obsidian_robot_regex.captures(lines.next().unwrap()).unwrap();
        let obsidian_robot_cost_ore: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let obsidian_robot_cost_clay: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let captures = geode_robot_regex.captures(lines.next().unwrap()).unwrap();
        let geode_robot_cost_ore: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let geode_robot_cost_clay: u32 = captures.get(2).unwrap().as_str().parse().unwrap();

        blueprints.push(Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost_ore,
            obsidian_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_clay,
        });

        // Consume empty line before next blueprint
        lines.next();
    }
    println!("blueprints = {:?}", blueprints);

    
}